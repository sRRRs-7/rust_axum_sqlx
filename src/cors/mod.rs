use crate::repository::create_repo;
use crate::router::router;
use axum::{
    http::header::CONTENT_TYPE,
    extract::Extension,
    Router,
};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer, AllowOrigin};


pub fn cors() -> CorsLayer {
    let swagger_url = "http://127.0.0.0::7878";
    CorsLayer::new()
        .allow_origin(AllowOrigin::exact(swagger_url.parse().unwrap()))
        .allow_methods(Any)
        .allow_headers(vec![CONTENT_TYPE])
}


pub async fn create_app() -> Router {
    let repositories = Arc::new(create_repo().await);
    router()
        .layer(cors())
        .layer(Extension(repositories))     // between router sharing state instance
}

