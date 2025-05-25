use http::Method;
use tonic_web::GrpcWebLayer;
use tower_http::cors::{Any, CorsLayer};

/// @ref https://connectrpc.com/docs/cors#configurations-by-protocol
pub fn cors() -> CorsLayer {
    tower_http::cors::CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
        .allow_origin([
            // local web ui
            "http://127.0.0.1:4000".parse().unwrap(),
            "http://localhost:4000".parse().unwrap(),
            // prod origins
            // "https://othi.dev".parse().unwrap(),
        ])
}

pub fn grpc_web() -> GrpcWebLayer {
    GrpcWebLayer::new()
}
