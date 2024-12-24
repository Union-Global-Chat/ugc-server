use sqlx::MySqlPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<MySqlPool>,
}

impl AppState {
    pub async fn connect(uri: &str) -> anyhow::Result<Self> {
        let pool = MySqlPool::connect(uri).await?;
        Ok(Self { pool: Arc::new(pool) })
    }
}