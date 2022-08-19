use axum::{body::Body, http::Request, response::Response, Router};
use tower::util::ServiceExt;

pub mod mock;
pub mod repository;

pub async fn request(
    router: Router,
    uri: &'static str,
    body: Body
) -> Response {
    router.oneshot(Request::builder().uri(uri).body(body).unwrap())
        .await
        .expect("Error request test")
}