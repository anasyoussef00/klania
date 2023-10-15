use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Sqlite, SqlitePool};
use tauri::AppHandle;

#[derive(Debug)]
pub struct Db {
    filename: String,
}

impl Db {
    pub fn new(filename: String) -> Self {
        Self { filename }
    }

    async fn get_db_url(&self, app_handle: &AppHandle) -> sqlx::Result<String> {
        let local_data_path = match app_handle.path_resolver().app_local_data_dir() {
            Some(path) => path,
            None => panic!("Could not find your local data directory!"),
        };
        let filename = &self.filename;
        let display_data_path = local_data_path.display();

        let url = format!("{display_data_path}/{filename}");

        if !Sqlite::database_exists(&url).await.unwrap_or(false) {
            Sqlite::create_database(&url).await?;
        }

        Ok(url)
    }

    async fn connect(db_url: &str) -> sqlx::Result<SqlitePool> {
        Ok(SqlitePoolOptions::new()
            .max_connections(10)
            .connect(db_url)
            .await?)
    }

    async fn migrate(conn: &SqlitePool) -> sqlx::Result<()> {
        Ok(sqlx::migrate!().run(conn).await?)
    }

    pub async fn init(&self, app_handle: &AppHandle) -> SqlitePool {
        let db_url = self
            .get_db_url(app_handle)
            .await
            .expect("Could not retrieve DB URL!");

        let conn = Self::connect(db_url.as_str())
            .await
            .expect("Could not create a connection!");

        Self::migrate(&conn).await.expect("Could not migrate!");

        conn
    }
}
