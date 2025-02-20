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
