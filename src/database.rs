use std::{env, error::Error};

use sqlx::MySqlPool;

pub async fn setup_database() -> Result<MySqlPool, Box<dyn Error>> {
    let database_url = env::var("DATABASE_URL").expect("$DATABASE_URL is not set");

    let db_pool = MySqlPool::connect(&database_url).await.unwrap();

    Ok(db_pool)
}
