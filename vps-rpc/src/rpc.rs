pub const REPO_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("repo_descriptor");
pub const SERVICE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("service_descriptor");

pub mod service {
    tonic::include_proto!("service");
    tonic::include_proto!("repo");
}

pub mod types {
    pub mod build {
        tonic::include_proto!("types.build");
    }
    pub mod deployment {
        tonic::include_proto!("types.deployment");
    }
}
