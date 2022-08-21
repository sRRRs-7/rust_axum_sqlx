use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub category_id: i32,
    pub titles: String,
    pub content: String,
}

pub type PostList = Vec<Post>;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct NewPost {
    pub user_id: i32,
    pub category_id: i32,
    pub titles: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct PostId {
    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct PostFindTitle {
    pub titles: String
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct PostFindContent {
    pub content: String
}
