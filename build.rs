fn main()-> protokit_build::Result<()> {

    protokit_build::Build::new()
        .compile("fscp.proto")?
        .out_dir("gen")
        .generate()
}