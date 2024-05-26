use sqlx::SqlitePool;
use tokio::sync::Mutex;

#[derive(Default)]
pub struct KlaniaState {
    pub db: Mutex<Option<SqlitePool>>
}
