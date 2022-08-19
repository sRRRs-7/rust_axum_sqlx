use crate::error::{AppError, Result};
use crate::models::user::{User, NewUser, UserCondition, UserId, UserList};
use crate::repository::RepoExt;
use crate::services::img_upload;
use crate::usecase;
use anyhow::anyhow;
use axum::{
    extract::{ContentLengthLimit, Extension, Multipart, Path, Query},
    http::StatusCode,
    Json
};


pub async fn index(
    Query(conditions): Query<UserCondition>,
    Extension(repo): RepoExt,
) -> Result<Json<UserList>> {
    let users = usecase::user::get_all(repo.clone(), &conditions).await?;
    Ok(Json(users))
}

pub async fn view() {}