// @see https://github.com/hyperium/tonic/issues/1643

pub mod derived;
pub mod impls;

pub const DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("descriptor");

pub type TonicResult<T> = Result<tonic::Response<T>, tonic::Status>;

tonic::include_proto!("helloworld");

pub mod service {
    tonic::include_proto!("service");
    tonic::include_proto!("repo");
    tonic::include_proto!("health");
}

pub mod blog {
    pub mod tag {
        tonic::include_proto!("blog.tag");
    }
    pub mod meta {
        tonic::include_proto!("blog.meta");
    }
    pub mod root {
        tonic::include_proto!("blog.root");
    }
}

pub mod common {
    pub mod db {
        tonic::include_proto!("common.db");
    }
}

pub mod types {
    pub mod build {
        tonic::include_proto!("types.build");
    }
    pub mod deployment {
        tonic::include_proto!("types.deployment");
    }
}
