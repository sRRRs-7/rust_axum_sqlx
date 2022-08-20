
use crate::handler::{root, users, categories, posts};
use axum::{
    routing::{get, post},
    Router,
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/users", user_routes())
        .nest("/categories", category_routes())
        .nest("/posts", post_routes())
}


fn user_routes() -> Router {
    Router::new()
        .route("/", get(users::list))
        .route("/detail/:user_id", get(users::detail))
        .route("/add", post(users::add))
        .route("/edit", post(users::edit))
        .route("/edit/image", post(users::edit_img))
        .route("/delete", post(users::delete))
}


fn category_routes() -> Router {
    Router::new()
        .route("/", get(categories::list))
        .route("/detail/:category_id", get(categories::detail))
        .route("/add", post(categories::add))
        .route("/edit", post(categories::edit))
        .route("/delete", post(categories::delete))
}


fn post_routes() -> Router {
    Router::new()
        .route("/", get(posts::list))
        .route("/detail/:post_id", get(posts::detail))
        .route("/add", post(posts::add))
        .route("/edit", post(posts::edit))
        .route("/delete", post(posts::delete))
}