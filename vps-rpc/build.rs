use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // let list = std::fs::read_dir(out_dir.clone()).unwrap();
    // let list: Vec<PathBuf> = list.into_iter().map(|e| e.unwrap().path()).collect();

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("helloworld_descriptor.bin"))
        .file_descriptor_set_path(out_dir.join("service_descriptor.bin"))
        .compile_protos(
            &[
                "proto/helloworld.proto",
                "proto/service.proto",
                "proto/types/build.proto",
                "proto/types/deployment.proto",
            ],
            &["proto"],
        )?;

    Ok(())
}
