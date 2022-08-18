
pub mod users;
pub mod categories;
pub mod posts;

pub async fn root() -> String {
    let st = String::from("Hello Root");
    st
}


#[cfg(test)]
mod tests {
    use crate::router;
    use crate::tests::request;
    use axum::{body::Body, http::StatusCode};

    #[tokio::test]
    async fn root() {
        let app = router::router();
        let res = request(app, "/", Body::empty()).await;
        assert_eq!(res.status(), StatusCode::OK);
    }
}