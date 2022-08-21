
use crate::handler::{root, users, categories, posts};
use axum::{
    routing::{get, post, put, delete},
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
        .route("/edit/image", post(users::edit_img))
        .route("/edit/:user_id", put(users::edit))
        .route("/delete", delete(users::delete))
}


fn category_routes() -> Router {
    Router::new()
        .route("/", get(categories::list))
        .route("/detail/:category_id", get(categories::detail))
        .route("/add", post(categories::add))
        .route("/edit/:category_id", put(categories::edit))
        .route("/delete", delete(categories::delete))
}


fn post_routes() -> Router {
    Router::new()
        .route("/", get(posts::list))
        .route("/find/user_id/:user_id", get(posts::find_by_user_id))
        .route("/find/category_id/:category_id", get(posts::find_by_category_id))
        .route("/find/titles", get(posts::find_by_titles))
        .route("/find/content", get(posts::find_by_content))
        .route("/detail/:post_id", get(posts::detail))
        .route("/add", post(posts::add))
        .route("/edit/:post_id", put(posts::edit))
        .route("/delete/:post_id", delete(posts::delete))
}