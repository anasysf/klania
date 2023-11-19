use crate::MIGRATIONS;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::{Sqlite, SqliteConnection};
use diesel_migrations::MigrationHarness;
use std::path::{Path, PathBuf};
use std::{ffi, fs, io};
use tauri::AppHandle;

static DB_NAME: &str = "klania";
static DB_EXT: &str = "sqlite3";

pub struct DB<'a> {
    app_handle: &'a AppHandle,
}

impl<'a> DB<'a> {
    pub fn new(app_handle: &'a AppHandle) -> Self {
        Self { app_handle }
    }

    pub fn establish_connection(&'a self) -> io::Result<Pool<ConnectionManager<SqliteConnection>>> {
        let db_local_dir = self.get_or_create_db_local_dir()?;
        let mut db_full_path = PathBuf::from(db_local_dir);
        db_full_path.push(DB_NAME);
        db_full_path.set_extension(DB_EXT);

        // TODO: SE A DEFAULT PATH IF CAN'T CONVERT
        let db_full_path = db_full_path
            .to_str()
            .expect("COULD NOT CONVERT THE DATABASE FULL PATH INTO A STRING!")
            .to_owned();

        let manager = ConnectionManager::<SqliteConnection>::new(db_full_path);

        Ok(Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            .expect("FAILED TO CREATE POOL!"))
    }

    pub fn run_migrations(conn: &'a mut impl MigrationHarness<Sqlite>) -> io::Result<()> {
        conn.run_pending_migrations(MIGRATIONS)
            .expect("COULD NOT RUN PENDING MIGRATIONS!");

        println!("MIGRATED SUCCESSFULLY!");

        Ok(())
    }

    fn get_or_create_db_local_dir(&'a self) -> io::Result<ffi::OsString> {
        match self.get_db_local_dir() {
            Some(path_buf) => {
                if !path_buf.as_path().exists() {
                    Self::create_db_local_dir(path_buf.clone())?;
                }

                Ok(path_buf.into_os_string())
            }
            None => Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "CAN'T DETECT THE LOCAL DATA DIRECTORY ON YOUR SYSTEM! IT'S PROBABLY UNSUPPORTED.",
            )),
        }
    }

    fn get_db_local_dir(&'a self) -> Option<PathBuf> {
        self.app_handle.path_resolver().app_local_data_dir()
    }

    fn create_db_local_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
        fs::create_dir_all(path)
    }
}
