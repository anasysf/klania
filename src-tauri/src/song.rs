use crate::state::KlaniaState;

use std::ffi::OsStr;
use std::io::{Read, BufReader};
use std::fs::File;
use std::path::Path;
use sqlx::SqlitePool;
use tauri::api::dialog::{MessageDialogBuilder, FileDialogBuilder, MessageDialogKind};
use tauri::{AppHandle, Manager, State};
use ring::digest::{SHA256, Context, Digest};
use data_encoding::HEXUPPER;

#[derive(Debug, sqlx::FromRow)]
pub struct Song {
    id: i32,
    title: Box<str>,
    path: Box<str>,
}

fn sha256_digest<R: Read>(mut reader: R) -> std::io::Result<Digest> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

fn hash<P: AsRef<Path>>(file_path: P) -> std::io::Result<Box<str>> {
    let input = File::open(file_path)?;
    let reader = BufReader::new(input);
    let digest = sha256_digest(reader)?;

    Ok(HEXUPPER.encode(digest.as_ref()).into())
}

impl Song {

    /// TODO: HANDLE BETTER.
    pub fn insert_song_from_menu(app_handle: AppHandle) {
        const ACCEPTED_EXTS: &[&str] = &["mp3"];

        FileDialogBuilder::new()
            .add_filter("Audio", ACCEPTED_EXTS)
            .set_title("Pick a song")
            .pick_file(move |path| {
                if let Some(path) = path {
                    tokio::spawn(async move {
                        let file_stem = path.file_stem().unwrap_or(OsStr::new("UNKNOWN"));

                        if let Some(path) = path.to_str() {
                            let title = file_stem.to_str().unwrap_or("UNKNOWN").into();

                            let state: State<KlaniaState> = app_handle.state();
                            let db_guard = state.db.lock().await;
                            let db = db_guard.as_ref().unwrap();

                            let file_hash = hash(path).expect("Could not hash file");
                            match Self::insert(db, title, path.into(), file_hash).await {
                                Ok(song) => println!("{song:?}"),
                                Err(err) => {
                                    if let Some(err) = err.into_database_error() {
                                        if err.is_unique_violation() {
                                            MessageDialogBuilder::new("Oops..!", "This song already exists.")
                                                .kind(MessageDialogKind::Error)
                                                .show(|_| {});
                                        } else {
                                            MessageDialogBuilder::new(err.code().unwrap(), err.message())
                                                .kind(MessageDialogKind::Error)
                                                .show(|_| {});
                                        }
                                    } else {
                                        MessageDialogBuilder::new("Oops..!", "An error has occurred while trying to save the song to the database.")
                                            .kind(MessageDialogKind::Error)
                                            .show(|_| {});
                                    }
                                }
                            };
                        } else {
                            MessageDialogBuilder::new("Oop..!", "The path provided is not UTF-8 encoded.")
                                .kind(MessageDialogKind::Error)
                                .show(|_| {});
                        };
                    });
                }
            });
    }

    pub async fn insert(
        db: &SqlitePool,
        title: Box<str>,
        path: Box<str>,
        file_hash: Box<str>,
    ) -> sqlx::Result<Self> {
        const INSERT_SONG_QUERY: &str = "INSERT INTO songs (title, path, file_hash) VALUES (?, ?, ?) RETURNING *";

        Ok(
            sqlx::query_as(INSERT_SONG_QUERY)
                .bind(title)
                .bind(path)
                .bind(file_hash)
                .fetch_one(db)
            .await?
        )
    }
}
