use crate::error::Result;
use crate::models::category::{Category, CategoryList, NewCategory, CategoryCondition };
use crate::repository::RepoExt;
use crate::usecase;
use axum::{
    extract::{Extension, Query, Path},
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

pub async fn edit(
    Path(category_id): Path<i32>,
    Json(new_category): Json<NewCategory>,
    Extension(repo): RepoExt,
) -> Result<Json<Category>> {
    let category = usecase::category::edit(repo.clone(), category_id, new_category).await?;
    Ok(Json(category))
}

pub async fn delete(
    Path(category_id): Path<i32>,
    Extension(repo): RepoExt,
) -> Result<Json<String>> {
    let res = usecase::category::delete(repo.clone(), category_id).await?;
    Ok(Json(res))
}