#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unreachable_patterns)]
#![allow(clippy::module_inception)]
use protokit::*;
pub fn register_types(registry: &mut protokit::textformat::reflect::Registry) {
    registry.register(&Init::default());
    registry.register(&File::default());
    registry.register(&Files::default());
    registry.register(&Chunk::default());
    registry.register(&Chunks::default());
    registry.register(&Request::default());
    registry.register(&More::default());
    registry.register(&Delete::default());
    registry.register(&Create::default());
    registry.register(&Data::default());
    registry.register(&Copy::default());
    registry.register(&Response::default());
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Algo(pub u32);
#[protoenum]
impl Algo {
    #[var(0u32, "SHA1")]
    pub const SHA1: Algo = Algo(0u32);
    #[var(1u32, "SHA256")]
    pub const SHA256: Algo = Algo(1u32);
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "Init", package = "fscp")]
pub struct Init {
    #[field(1u32, "algo", protoenum, singular)]
    pub algo: Algo,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "File", package = "fscp")]
pub struct File {
    #[field(1u32, "path", string, singular)]
    pub path: String,
    #[field(3u32, "len", varint, singular)]
    pub len: u64,
    #[field(2u32, "mtime", varint, singular)]
    pub mtime: u64,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "Files", package = "fscp")]
pub struct Files {
    #[field(1u32, "file", nested, repeated)]
    pub file: Vec<File>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "Chunk", package = "fscp")]
pub struct Chunk {
    #[field(1u32, "hash", bytes, singular)]
    pub hash: Vec<u8>,
    #[field(2u32, "pos", varint, singular)]
    pub pos: u64,
    #[field(3u32, "len", varint, singular)]
    pub len: u32,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "Chunks", package = "fscp")]
pub struct Chunks {
    #[field(1u32, "path", string, singular)]
    pub path: String,
    #[field(2u32, "hashes", nested, repeated)]
    pub hashes: Vec<Chunk>,
}
#[derive(Debug, Clone, PartialEq, Proto)]
pub enum RequestOneOfKind {
    #[field(1u32, "init", nested, raw)]
    Init(Init),
    #[field(2u32, "files", nested, raw)]
    Files(Files),
    #[field(3u32, "chunk", nested, raw)]
    Chunk(Chunks),
    __Unused(::core::marker::PhantomData<&'static ()>),
}
impl Default for RequestOneOfKind {
    fn default() -> Self {
        Self::Init(Default::default())
    }
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "Request", package = "fscp")]
pub struct Request {
    #[oneof([1u32, 2u32, 3u32], ["init", "files", "chunk"])]
    pub Kind: Option<RequestOneOfKind>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "More", package = "fscp")]
pub struct More {
    #[field(1u32, "file", string, repeated)]
    pub file: Vec<String>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "Delete", package = "fscp")]
pub struct Delete {
    #[field(1u32, "file", string, singular)]
    pub file: String,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "Create", package = "fscp")]
pub struct Create {
    #[field(1u32, "file", string, singular)]
    pub file: String,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "Data", package = "fscp")]
pub struct Data {
    #[field(1u32, "pos", varint, singular)]
    pub pos: u64,
    #[field(2u32, "len", varint, singular)]
    pub len: u32,
    #[field(3u32, "data", bytes, singular)]
    pub data: Vec<u8>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "Copy", package = "fscp")]
pub struct Copy {
    #[field(1u32, "src", string, singular)]
    pub src: String,
    #[field(2u32, "dst", string, singular)]
    pub dst: String,
}
#[derive(Debug, Clone, PartialEq, Proto)]
pub enum ResponseOneOfKind {
    #[field(1u32, "more", nested, raw)]
    More(More),
    #[field(2u32, "delete", nested, raw)]
    Delete(Delete),
    #[field(3u32, "create", nested, raw)]
    Create(Create),
    #[field(4u32, "data", nested, raw)]
    Data(Data),
    #[field(5u32, "copy", nested, raw)]
    Copy(Copy),
    #[field(6u32, "done", bool, raw)]
    Done(bool),
    __Unused(::core::marker::PhantomData<&'static ()>),
}
impl Default for ResponseOneOfKind {
    fn default() -> Self {
        Self::More(Default::default())
    }
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
#[proto(name = "Response", package = "fscp")]
pub struct Response {
    #[oneof(
        [1u32,
        2u32,
        3u32,
        4u32,
        5u32,
        6u32,
        ],
        ["more",
        "delete",
        "create",
        "data",
        "copy",
        "done",
        ]
    )]
    pub Kind: Option<ResponseOneOfKind>,
}
mod Fscp_server {
    use super::*;
    use protokit::grpc::*;
    #[protokit::grpc::async_trait]
    pub trait Fscp: Send + Sync + 'static {
        type SyncStream: Stream<Item = Result<super::Response, Status>> + Send + 'static;
        async fn sync(
            &self,
            req: tonic::Request<tonic::Streaming<super::Request>>,
        ) -> Result<tonic::Response<Self::SyncStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct FscpServer<S: Fscp>(pub Arc<S>);
    impl<S: Fscp> Clone for FscpServer<S> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<S: Fscp> From<S> for FscpServer<S> {
        fn from(v: S) -> Self {
            Self(::std::sync::Arc::new(v))
        }
    }
    impl<S: Fscp> From<::std::sync::Arc<S>> for FscpServer<S> {
        fn from(v: ::std::sync::Arc<S>) -> Self {
            Self(v)
        }
    }
    struct SyncSvc<S: Fscp>(Arc<S>);
    impl<S: Fscp> tonic::server::StreamingService<super::Request> for SyncSvc<S> {
        type Response = super::Response;
        type ResponseStream = S::SyncStream;
        type Future = BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
        fn call(
            &mut self,
            request: tonic::Request<tonic::Streaming<super::Request>>,
        ) -> Self::Future {
            let inner = self.0.clone();
            Box::pin(async move { inner.sync(request).await })
        }
    }
    impl<S, B> Service<http::Request<B>> for FscpServer<S>
    where
        S: Fscp,
        B: Body + Send + 'static,
        B::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>> + Send
            + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = core::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/fscp.Fscp/Sync" => {
                    let inner = self.0.clone();
                    let fut = async move {
                        let method = SyncSvc(inner);
                        let codec = protokit::grpc::TonicCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<S: Fscp> tonic::transport::NamedService for FscpServer<S> {
        const NAME: &'static str = "fscp.Fscp";
    }
}
pub use Fscp_server::*;
mod Fscp_client {
    use super::*;
    use protokit::grpc::*;
    #[derive(Debug, Clone)]
    pub struct FscpClient<C> {
        inner: tonic::client::Grpc<C>,
    }
    impl FscpClient<tonic::transport::Channel> {
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: core::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<S> FscpClient<S>
    where
        S: tonic::client::GrpcService<tonic::body::BoxBody>,
        S::Error: Into<StdError>,
        S::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <S::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: S) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: S,
            interceptor: F,
        ) -> FscpClient<InterceptedService<S, F>>
        where
            F: tonic::service::Interceptor,
            S::ResponseBody: Default,
            S: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <S as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <S as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            FscpClient::new(InterceptedService::new(inner, interceptor))
        }
        pub async fn sync(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::Request>,
        ) -> Result<tonic::Response<tonic::Streaming<super::Response>>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    Status::new(
                        Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = protokit::grpc::TonicCodec::default();
            let path = http::uri::PathAndQuery::from_static("/fscp.Fscp/Sync");
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
    }
}
pub use Fscp_client::*;
