use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;
use std::sync::Arc;

pub type Db = Arc<Pool<Postgres>>;

pub async fn db_connect() -> Pool<Postgres> {
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("Couldn't find DATABASE_URL in env var");

    println!("connect postgres database..");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Error connecting to database")


}