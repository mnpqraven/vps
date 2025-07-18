use tonic::{Response, Status};

pub mod error;

pub type TonicResult<T> = Result<Response<T>, Status>;
