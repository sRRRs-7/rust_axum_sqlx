use crate::error::{AppError, Result};
use crate::models::user::{User, NewUser, UserCondition, UserId, UserList};
use crate::repository::RepoExt;
use crate::services::img_upload::img_upload;
use crate::usecase;
use anyhow::anyhow;
use axum::{
    extract::{ContentLengthLimit, Extension, Multipart, Path, Query},
    http::StatusCode,
    Json
};


pub async fn list(
    Query(conditions): Query<UserCondition>,
    Extension(repo): RepoExt,
) -> Result<Json<UserList>> {
    let users = usecase::user::get_all(repo.clone(), &conditions).await?;
    Ok(Json(users))
}

pub async fn detail(
    Path(user_id): Path<i32>,
    Extension(repo): RepoExt,
) -> Result<Json<User>> {
    let user = usecase::user::get(repo.clone(), user_id).await?;
    Ok(Json(user))
}

pub async fn add(
    Json(new_user): Json<NewUser>,
    Extension(repo): RepoExt,
) -> Result<Json<UserId>> {
    let user_id = usecase::user::add(repo.clone(), new_user).await?;
    Ok(Json(user_id))
}

pub async fn edit_img(
    ContentLengthLimit(multipart): ContentLengthLimit<Multipart, {5 * 1024 * 1024}>,
    Extension(repo): RepoExt,
) -> Result<Json<User>> {
    let result = _multipart_edit_img(multipart).await;
    if let Err(e) = result {
        return Err(AppError::MultipartError(e.to_string()));
    }

    let (user_id, img) = result.unwrap();
    let result = img_upload(img, "/");
    println!("user_id: {}, upload img: {:?}", user_id, result);
    let user = usecase::user::get(repo.clone(), user_id).await?;
    Ok(Json(user))
}

async fn _multipart_edit_img(mut multipart: Multipart) -> Result<(i32, Vec<u8>), anyhow::Error> {
    let mut user_id = None;
    let mut img = Vec::new();

    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap_or("").to_string();
        let data: Vec<u8> = field.bytes().await?.into_iter().collect();

        match &*name {
            "user_id" => user_id = Some(std::str::from_utf8(&data)?.parse()?),
            "img" => img = data,
            _ => return Err(anyhow!("Invalid parameter")),
        }
    }
    Ok((user_id.unwrap(), img))
}

pub async fn edit() -> StatusCode {
    StatusCode::OK
}

pub async fn delete() -> StatusCode {
    StatusCode::OK
}