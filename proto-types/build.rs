use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let root_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let proto_dir = root_dir.join("proto");
    let proto_files: Vec<PathBuf> = proto_build_help::list_proto_files(&proto_dir)
        .into_iter()
        .map(|file| file.strip_prefix(root_dir.clone()).unwrap().to_path_buf())
        .collect();

    #[cfg(feature = "transport")]
    tonic_build::configure()
        .type_attribute(
            ".",
            "#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)] #[serde(rename_all = \"snake_case\")]",
        )
        .file_descriptor_set_path(out_dir.join("descriptor.bin"))
        .compile_protos(&proto_files, &["proto"])?;

    #[cfg(not(feature = "transport"))]
    tonic_build::configure()
        .type_attribute(
            ".",
            "#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)] #[serde(rename_all = \"snake_case\")]",
        )
        .build_client(false)
        .build_server(false)
        .build_transport(false)
        .file_descriptor_set_path(out_dir.join("descriptor.bin"))
        .compile_protos(&proto_files, &["proto"])?;

    Ok(())
}
