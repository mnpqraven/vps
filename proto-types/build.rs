use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let root_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let proto_dir = root_dir.join("proto");
    let proto_files: Vec<PathBuf> = proto_build_help::list_proto_files(&proto_dir)
        .into_iter()
        .map(|file| file.strip_prefix(root_dir.clone()).unwrap().to_path_buf())
        .collect();

    #[allow(unused_mut, reason = "need for feature")]
    #[allow(unused_assignments)]
    let mut transport = true;
    #[cfg(not(feature = "transport"))]
    {
        transport = false;
    }

    tonic_build::configure()
        .type_attribute(
            ".",
            "#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema, ts_rs::TS)] #[ts(export, optional_fields)] #[serde(rename_all = \"camelCase\")]",
        )
        .type_attribute("common.db.ProtoPagination", "#[derive(utoipa::IntoParams)]")
        .build_client(transport)
        .build_server(transport)
        .build_transport(transport)
        .file_descriptor_set_path(out_dir.join("descriptor.bin"))
        .compile_protos(&proto_files, &["proto"])?;

    Ok(())
}
