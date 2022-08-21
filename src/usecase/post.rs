use crate::error::Result;
use crate::models::post::{
    Post,
    NewPost,
    PostId,
    PostList,
    PostFindTitle,
    PostFindContent,
};
use crate::repository::{post::PostRepoTrait, Repositories};
use std::sync::Arc;


pub async fn find_all <R: Repositories>(
    repo: Arc<R>,
) -> Result<PostList> {
    let posts = repo.post().find_all().await?;
    Ok(posts)
}

pub async fn find_by_user_id <R: Repositories>(
    repo: Arc<R>,
    user_id: i32,
) -> Result<PostList> {
    let posts = repo.post().find_by_user_id(user_id).await?;
    Ok(posts)
}

pub async fn find_by_category_id <R: Repositories>(
    repo: Arc<R>,
    category_id: i32,
) -> Result<PostList> {
    let posts = repo.post().find_by_category_id(category_id).await?;
    Ok(posts)
}

pub async fn find_by_titles <R: Repositories>(
    repo: Arc<R>,
    titles: PostFindTitle,
) -> Result<PostList> {
    let posts = repo.post().find_by_titles(&titles).await?;
    Ok(posts)
}

pub async fn find_by_content <R: Repositories>(
    repo: Arc<R>,
    content: PostFindContent,
) -> Result<PostList> {
    let posts = repo.post().find_by_content(&content).await?;
    Ok(posts)
}

pub async fn find_by_id <R: Repositories>(
    repo: Arc<R>,
    post_id: i32,
) -> Result<Post> {
    let post = repo.post().find_by_id(post_id).await?;
    Ok(post)
}

pub async fn add <R: Repositories>(
    repo: Arc<R>,
    new_post: NewPost,
) -> Result<PostId> {
    let post_id = repo.post().add(&new_post).await?;
    Ok(post_id)
}

pub async fn edit <R: Repositories>(
    repo: Arc<R>,
    post_id: i32,
    new_post: NewPost,
) -> Result<PostId> {
    let post_id = repo.post().edit(post_id, &new_post).await?;
    Ok(post_id)
}

pub async fn delete <R: Repositories>(
    repo: Arc<R>,
    post_id: i32,
) -> Result<String> {
    let res = repo.post().delete(post_id).await?;
    Ok(res)
}
