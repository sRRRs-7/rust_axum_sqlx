use crate::error::Result;
use crate::models::post::{
    Post,
    NewPost,
    PostId,
    PostList,
    PostFindTitle,
    PostFindContent,
};
use crate::repository::RepoExt;
use crate::usecase;
use axum::extract::Query;
use axum::{
    extract::{Extension, Path},
    Json
};


pub async fn list(
    Extension(repo): RepoExt
) -> Result<Json<PostList>> {
    let posts = usecase::post::find_all(repo.clone()).await?;
    Ok(Json(posts))
}

pub async fn find_by_user_id(
    Path(user_id): Path<i32>,
    Extension(repo): RepoExt
) -> Result<Json<PostList>>  {
    let posts = usecase::post::find_by_user_id(repo.clone(), user_id).await?;
    Ok(Json(posts))
}

pub async fn find_by_category_id(
    Path(category_id): Path<i32>,
    Extension(repo): RepoExt
) -> Result<Json<PostList>> {
    let posts = usecase::post::find_by_category_id(repo.clone(), category_id).await?;
    Ok(Json(posts))
}

pub async fn find_by_titles(
    Query(titles): Query<PostFindTitle>,
    Extension(repo): RepoExt
) -> Result<Json<PostList>> {
    let posts = usecase::post::find_by_titles(repo.clone(), titles).await?;
    Ok(Json(posts))
}

pub async fn find_by_content(
    Query(content): Query<PostFindContent>,
    Extension(repo): RepoExt
) -> Result<Json<PostList>> {
    let posts = usecase::post::find_by_content(repo.clone(), content).await?;
    Ok(Json(posts))
}

pub async fn detail(
    Path(post_id): Path<i32>,
    Extension(repo): RepoExt,
) -> Result<Json<Post>> {
    let post = usecase::post::find_by_id(repo.clone(), post_id).await?;
    Ok(Json(post))
}

pub async fn add(
    Json(body): Json<NewPost>,
    Extension(repo): RepoExt,
) -> Result<Json<PostId>>  {
    let post_id = usecase::post::add(repo.clone(), body).await?;
    Ok(Json(post_id))
}

pub async fn edit(
    Path(post_id): Path<i32>,
    Json(body): Json<NewPost>,
    Extension(repo): RepoExt,
) -> Result<Json<PostId>>  {
    let post_id = usecase::post::edit(repo.clone(), post_id, body).await?;
    Ok(Json(post_id))
}

pub async fn delete(
    Path(post_id): Path<i32>,
    Extension(repo): RepoExt,
) -> Result<Json<String>> {
    let res = usecase::post::delete(repo.clone(), post_id).await?;
    Ok(Json(res))
}