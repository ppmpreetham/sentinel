use std::time::Duration;

use crate::config::config;
use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn pg_pool() -> PgPool {
    let url = &config().db_url;
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(3))
        .connect(url)
        .await
        .expect("fail to connect to db pool");
    pool
}
