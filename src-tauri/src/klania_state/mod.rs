use sqlx::SqlitePool;
use tokio::sync::Mutex;

#[derive(Debug, Default)]
pub struct KlaniaState {
    pub db: Mutex<Option<SqlitePool>>,
}
