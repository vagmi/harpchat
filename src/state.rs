#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AppState {
    pub db_pool: sqlx::PgPool,
}
