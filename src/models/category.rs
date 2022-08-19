use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Category {
    pub id: i32,
    pub category: String,
}

pub type CategoryList = Vec<Category>;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewCategory {
    pub category: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryCondition {
    pub category: Option<String>,
}