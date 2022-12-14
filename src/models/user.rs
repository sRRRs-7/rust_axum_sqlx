use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub msg: Option<String>,
    pub age: Option<i16>,
    pub image: Option<String>,
}
pub type UserList = Vec<User>;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUser {
    pub name: String,
    pub msg: Option<String>,
    pub age: Option<i16>,
    pub image: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserCondition {
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct UserId {
    pub id: i32,
}

#[derive(Serialize)]
pub struct ProfImg {
    pub user_id: i32,
    pub image: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct ImgUrl {
    pub image: String,
}