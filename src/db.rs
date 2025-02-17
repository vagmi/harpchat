use std::env;

use sqlx::{PgPool, postgres::PgPoolOptions};
use crate::error::Result;

pub async fn setup_db() -> Result<PgPool> {
    let database_url = env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new().max_connections(8).connect(&database_url).await?;

    Ok(pool)
}
