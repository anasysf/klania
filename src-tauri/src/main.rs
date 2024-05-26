// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::song::Song;

use state::KlaniaState;
use tauri::{Manager, State};

mod db;
mod state;
mod menu;
mod song;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .manage(KlaniaState::default())
        .setup(|app| {
            let app_handle = app.app_handle();

            tokio::spawn(async move {
                let state: State<KlaniaState> = app_handle.state();

                let db = db::initialize(&app_handle).await;
                db::migrate(&db.clone().unwrap()).await.unwrap();

                let mut db_guard = state.db.lock().await;
                *db_guard = db;
            });

            Ok(())
        })
        .menu(menu::make_menu())
        .on_menu_event(move |event| {
            let window = event.window();
            let app_handle = window.app_handle();

            match event.menu_item_id() {
                "open-file" => Song::insert_song_from_menu(app_handle),
                _ => unreachable!(),
            }
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
