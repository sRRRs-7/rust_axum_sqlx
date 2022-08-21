use std::net::{SocketAddr};

mod cors;
mod db;
mod error;
mod handler;
mod models;
mod repository;
mod router;
mod services;
mod usecase;
#[cfg(test)]
mod tests;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let addr = SocketAddr::from(([127,0,0,1], 7878));
    tracing::debug!("listening on {}", addr);

    let app = cors::create_app().await;
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
