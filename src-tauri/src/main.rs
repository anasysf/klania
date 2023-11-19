// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod db;
pub mod schema;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            use db::DB;

            let app_handle = app.handle();

            let db = DB::new(&app_handle);

            let conn = db.establish_connection().unwrap();

            let mut conn = conn.get().expect("COULD NOT GET THE CONNECTION");

            DB::run_migrations(&mut conn).unwrap();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
