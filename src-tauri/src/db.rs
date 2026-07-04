use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use tauri::{App, Manager};

pub(crate) struct Db(pub(crate) SqlitePool);

pub(crate) async fn init(app: &App) -> Result<SqlitePool, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Не удалось определить папку данных приложения: {e}"))?;
    std::fs::create_dir_all(&dir).map_err(|e| format!("Не удалось создать папку данных: {e}"))?;

    let db_path = dir.join("otdr-viewer.sqlite3");
    log::debug!("SQLite: {}", db_path.display());

    let options = SqliteConnectOptions::new().filename(&db_path).create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .connect_with(options)
        .await
        .map_err(|e| format!("Не удалось открыть базу данных: {e}"))?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| format!("Не удалось применить миграции: {e}"))?;

    Ok(pool)
}
