use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Local, Utc};
use shared_types::RecentFileEntry;
use sqlx::FromRow;
use tauri::State;

use crate::commands::sor::read_fiber_length_km;
use crate::db::Db;

const RECENT_FILES_LIMIT: i64 = 20;

#[derive(FromRow)]
struct RecentFileRow {
    path: String,
    name: String,
    opened_at: i64,
    length_km: Option<f64>,
}

impl From<RecentFileRow> for RecentFileEntry {
    fn from(row: RecentFileRow) -> Self {
        let location = Path::new(&row.path)
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();

        Self {
            path: row.path,
            name: row.name,
            location,
            opened_at_label: format_timestamp(row.opened_at),
            length_label: row.length_km.map(format_length_km),
        }
    }
}

fn now_unix() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

fn format_timestamp(unix_seconds: i64) -> String {
    let Some(utc) = DateTime::<Utc>::from_timestamp(unix_seconds, 0) else {
        return String::new();
    };
    utc.with_timezone(&Local).format("%d.%m.%Y").to_string()
}

fn format_length_km(km: f64) -> String {
    format!("{km:.1} км")
}

#[tauri::command]
pub(crate) async fn record_recent_file(db: State<'_, Db>, path: String) -> Result<(), ()> {
    let name = Path::new(&path)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.clone());

    let parse_path = path.clone();
    let length_km = tauri::async_runtime::spawn_blocking(move || read_fiber_length_km(&parse_path))
        .await
        .unwrap_or(None);

    let result = sqlx::query(
        r#"
            INSERT INTO recent_files (path, name, opened_at, length_km)
            VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT (path) DO UPDATE SET opened_at = excluded.opened_at
        "#,
    )
    .bind(&path)
    .bind(&name)
    .bind(now_unix())
    .bind(length_km)
    .execute(&db.0)
    .await;

    if let Err(e) = result {
        log::error!("record_recent_file: не удалось сохранить запись: {e}");
    }

    Ok(())
}

#[tauri::command]
pub(crate) async fn list_recent_files(db: State<'_, Db>) -> Result<Vec<RecentFileEntry>, ()> {
    let rows = sqlx::query_as::<_, RecentFileRow>(
        r#"
            SELECT path, name, opened_at, length_km
            FROM recent_files
            ORDER BY opened_at DESC
            LIMIT ?1
        "#,
    )
    .bind(RECENT_FILES_LIMIT)
    .fetch_all(&db.0)
    .await;

    match rows {
        Ok(rows) => Ok(rows.into_iter().map(RecentFileEntry::from).collect()),
        Err(e) => {
            log::error!("list_recent_files: не удалось получить список: {e}");
            Ok(Vec::new())
        }
    }
}

#[tauri::command]
pub(crate) async fn clear_recent_files(db: State<'_, Db>) -> Result<(), ()> {
    if let Err(e) = sqlx::query("DELETE FROM recent_files").execute(&db.0).await {
        log::error!("clear_recent_files: не удалось очистить список: {e}");
    }

    Ok(())
}
