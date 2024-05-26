use std::path::Path;

use sqlx::{migrate::{MigrateDatabase, Migrator}, sqlite::SqlitePoolOptions, Sqlite, SqlitePool};
use tauri::{api::dialog::{blocking::MessageDialogBuilder, MessageDialogKind}, AppHandle};

static MIGRATOR: Migrator = sqlx::migrate!();

pub async fn initialize(app_handle: &AppHandle) -> Option<SqlitePool> {
    let app_local_data_dir = app_handle.path_resolver().app_local_data_dir();
    match app_local_data_dir {
        Some(path) => {
            const DB_NAME: &str = "klania.sqlite3";
            let db_path = Path::new(&path).join(DB_NAME);
            create_db(db_path).await.ok()
        },
        None => {
            MessageDialogBuilder::new("Oops..!", "No local data directory found on your os.")
                .kind(MessageDialogKind::Error)
                .show();

            None
        },
    }
}

pub async fn migrate(db: &SqlitePool) -> sqlx::Result<()> {
    Ok(MIGRATOR.run(db).await?)
}

async fn create_db<P: AsRef<Path>>(db_path: P) -> sqlx::Result<SqlitePool> {
    let db_path = db_path
        .as_ref()
        .to_str()
        .unwrap_or_else(|| panic!("invalid unicode: {}", db_path.as_ref().display()));

    if !Sqlite::database_exists(db_path).await.unwrap_or(false) {
        Sqlite::create_database(db_path).await?;
    }

    SqlitePoolOptions::new()
        .connect(db_path)
        .await
}
