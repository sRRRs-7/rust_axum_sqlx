use crate::error::{AppError, Result};
use crate::models::post::{Post, NewPost, PostId, PostList};
use crate::repository::RepoExt;
use crate::repository::post::PostRepoTrait;
use anyhow::anyhow;
use axum::{
    extract::{ContentLengthLimit, Extension, Multipart, Path, Query},
    http::StatusCode,
    Json
};


pub async fn list(Extension(repo): RepoExt) -> Result<Json<PostList>> {
    let posts = repo.post.find_all().await?;
    Ok(Json(posts))
}

pub async fn detail() -> StatusCode {
    StatusCode::OK
}

pub async fn add() -> StatusCode  {
    StatusCode::OK
}

pub async fn edit() -> StatusCode  {
    StatusCode::OK
}

pub async fn delete() -> StatusCode  {
    StatusCode::OK
}