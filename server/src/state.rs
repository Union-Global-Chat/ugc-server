use sqlx::MySqlPool;
use std::sync::Arc;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<MySqlPool>,
    pub tx: broadcast::Sender<String>,
}

impl AppState {
    pub async fn connect(uri: &str) -> anyhow::Result<Self> {
        let pool = MySqlPool::connect(uri).await?;
        let (tx, _) = broadcast::channel(11);
        Ok(Self {
            pool: Arc::new(pool),
            tx,
        })
    }
}
