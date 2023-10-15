use sqlx::{Sqlite, SqlitePool};
use tauri::{AppHandle, Manager};

use crate::{entities::song::Song, klania_state::KlaniaState};

pub struct SongManager {
    db: SqlitePool,
    song: Song,
}

impl SongManager {
    pub async fn new(app_handle: &AppHandle, song: Song) -> Self {
        let app_state: tauri::State<KlaniaState> = app_handle.state();
        let db_lock = app_state.db.lock().await;
        let db = db_lock.as_ref().unwrap().to_owned();

        Self { db, song }
    }

    async fn get_all(&self) -> sqlx::Result<Vec<Song>> {
        todo!()
    }

    pub async fn get(&self, id: i64) -> sqlx::Result<Song> {
        static SQL: &str = "SELECT * FROM songs WHERE id = ?";

        Ok(sqlx::query_as::<Sqlite, Song>(SQL)
            .bind(id)
            .fetch_one(&self.db)
            .await?)
    }

    pub async fn insert(&self) -> sqlx::Result<i64> {
        static SQL: &str = "INSERT INTO songs (file_path, file_hash) VALUES ($1, $2)";

        Ok(sqlx::query(SQL)
            .bind(self.song.file_path.as_str())
            .bind(self.song.file_hash.as_str())
            .execute(&self.db)
            .await?
            .last_insert_rowid())
    }

    fn update(&self, new_song: Song) -> sqlx::Result<i64> {
        todo!()
    }

    fn delete(&self) -> sqlx::Result<()> {
        todo!()
    }
}
