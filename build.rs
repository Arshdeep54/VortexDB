fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=indexer/indexer_thread.proto");

    prost_build::compile_protos(&["indexer/indexer_thread.proto"], &["indexer/"])?;

    Ok(())
}
