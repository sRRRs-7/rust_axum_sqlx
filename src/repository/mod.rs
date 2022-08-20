use crate::db;
use crate::repository::{
    user::{UserRepo, UserRepoTrait},
    category::{CategoryRepo, CategoryRepoTrait},
    post::{PostRepo, PostRepoTrait},
};
use axum::extract::Extension;
use std::sync::Arc;

pub mod user;
pub mod category;
pub mod post;

pub type RepoExt = Extension<Arc<RepoImpls>>;

pub struct RepoImpls {
    pub user: UserRepo,
    pub category: CategoryRepo,
    pub post: PostRepo,
}
impl RepoImpls {
    pub fn new(
        user: UserRepo,
        category: CategoryRepo,
        post: PostRepo,
    ) -> Self {
        Self { user, category, post }
    }
}

pub async fn create_repo() -> RepoImpls {
    let pool = Arc::new(db::db_connect().await);
    RepoImpls::new(
        UserRepo::new(pool.clone()),
        CategoryRepo::new(pool.clone()),
        PostRepo::new(pool.clone()),
    )
}


pub trait Repositories {
    type UserRepo: UserRepoTrait;
    type CategoryRepo: CategoryRepoTrait;
    type PostRepo: PostRepoTrait;
    fn user(&self) -> &Self::UserRepo;
    fn category(&self) -> &Self::CategoryRepo;
    fn post(&self) -> &Self::PostRepo;
}

impl Repositories for RepoImpls {
    type UserRepo = UserRepo;
    type CategoryRepo = CategoryRepo;
    type PostRepo = PostRepo;
    fn user(&self) -> &Self::UserRepo {
        &self.user
    }
    fn category(&self) -> &Self::CategoryRepo {
        &self.category
    }
    fn post(&self) -> &Self::PostRepo {
        &self.post
    }
}