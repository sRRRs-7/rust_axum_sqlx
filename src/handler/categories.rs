use crate::error::{AppError, Result};
use crate::models::category::{Category, CategoryList, NewCategory, CategoryCondition };
use crate::repository::{RepoExt};
use crate::usecase;
use anyhow::anyhow;
use axum::{
    extract::{Extension, Query, Path},
    http::StatusCode,
    Json
};


pub async fn list(
    Query(conditions): Query<CategoryCondition>,
    Extension(repo): RepoExt
) -> Result<Json<CategoryList>> {
    let categories = usecase::category::get_all(repo.clone(), &conditions).await?;
    Ok(Json(categories))
}


pub async fn detail(
    Path(category_id): Path<i32>,
    Extension(repo): RepoExt,
) -> Result<Json<Category>> {
    let category = usecase::category::get(repo.clone(), category_id).await?;
    Ok(Json(category))
}

pub async fn add(
    Json(new_category): Json<NewCategory>,
    Extension(repo): RepoExt,
) -> Result<Json<Category>> {
    let category = usecase::category::add(repo.clone(), new_category).await?;
    Ok(Json(category))
}

pub async fn edit() -> StatusCode {
    StatusCode::OK
}

pub async fn delete() -> StatusCode {
    StatusCode::OK
}