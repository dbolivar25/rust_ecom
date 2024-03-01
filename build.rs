use std::error::Error;
use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = &PathBuf::from(env::var("OUT_DIR")?);

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("rust_ecom_descriptor.bin"))
        .compile(&["proto/rust_ecom.proto"], &["proto"])?;

    tonic_build::compile_protos("proto/rust_ecom.proto")?;

    println!("cargo:rerun-if-changed=migrations");

    Ok(())
}
