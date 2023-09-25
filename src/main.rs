pub mod gen {
    include!("../gen/mod.rs");
}

use std::iter::from_fn;
use std::path::Path;
use protokit::grpc::futures::stream::iter;
use protokit::grpc::futures::{SinkExt, StreamExt};
use futures::Stream;
use gen::fscp::fscp;
use crate::gen::fscp::fscp::{Init, RequestOneOfKind, ResponseOneOfKind};


fn check(root: &Path) ->  impl Iterator<Item=fscp::File> {
    let mut walkdir = walkdir::WalkDir::new(root).into_iter();
    from_fn(move || {
        match walkdir.next() {
            None => return None,
            Some(Ok(entry)) => {
                return Some(fscp::File {
                    path: entry.path().to_string_lossy().to_string(),
                    len: entry.metadata().unwrap().len(),
                    root: vec![],
                })
            }
            Some(Err(e)) => panic!("Err")
        }
    })
}


async fn server(src: String, dst: String) {

}

async fn client(src: String, dst: String) {

}

#[tokio::main]
async fn main() {
    let mut src = None;
    let mut dst = None;

    let mut args = std::env::args().skip(1);
    for arg in &mut args {
        match arg.as_str() {
            v if src.is_none() => {
                src = Some(v.to_string());
            }
            v if src.is_some() && dst.is_none() => { dst = Some(v.to_string()) }
            v => panic!("Multiple destinations provided"),
        }
    }

    let Some(src) = src else {
        panic!("No source provided");
    };

    let Some(dst) = dst else {
        panic!("No destination provided");
    };

    let mut client = fscp::FscpClient::connect("127.0.0.1:999").await.expect("Could not connect");

    let (tx, rx) = tokio::sync::oneshot::channel::<tonic::Response<_>>();
    let (finish, done) = tokio::sync::oneshot::channel();
    let local = async_fn_stream::fn_stream(|sink| async move {
        sink.emit(fscp::Request {
            Kind: Some(RequestOneOfKind::Init(Init {}))
        }).await;

        let mut remote: tonic::Streaming<fscp::Response> = rx.await.unwrap().into_inner();

        let item = loop {
            let item = match remote.next().await {
                None => panic!(""),
                Some(Ok(v)) => v,
                Some(Err(e)) => panic!("E"),
            };
            match item.Kind {
                Some(fscp::ResponseOneOfKind::More(more)) => {
                    // Send metadata
                }
                None => panic!(),
                Some(other) => break other
            }
        };

        let mut remote = iter([Ok(fscp::Response { Kind: Some(item) })])
            .chain(remote);

        loop {
            let item = match remote.next().await {
                None => panic!(""),
                Some(Ok(v)) => v,
                Some(Err(e)) => panic!("E"),
            };

            match item.Kind {
                None => panic!("Stream has ended"),
                Some(fscp::ResponseOneOfKind::More(m)) => panic!("Protocol error"),
                Some(fscp::ResponseOneOfKind::Create(create)) => {}
                Some(fscp::ResponseOneOfKind::Delete(delete)) => {}
                Some(fscp::ResponseOneOfKind::Copy(copy)) => {}
                Some(fscp::ResponseOneOfKind::Data(data)) => {}
                Some(fscp::ResponseOneOfKind::Done(done)) => {
                    break;
                }
                other => panic!("Wrong protocol kind: {:?}", other)
            }
        }
        finish.send(()).unwrap();
    });

    tx.send(client.sync(local).await.expect("Failed")).expect("failed to send");
    done.await.unwrap();
    // Go through a directory
    println!("Hello, world!");
}
