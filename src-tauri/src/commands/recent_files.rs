use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Local, Utc};
use serde::Serialize;
use sqlx::Row;
use tauri::State;

use crate::db::Db;

const RECENT_FILES_LIMIT: i64 = 20;

#[derive(Serialize, Debug)]
pub(crate) struct RecentFileEntry {
    path: String,
    name: String,
    location: String,
    opened_at_label: String,
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

#[tauri::command]
pub(crate) async fn record_recent_file(db: State<'_, Db>, path: String) -> Result<(), ()> {
    let name = Path::new(&path)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.clone());

    let result = sqlx::query(
        r#"
                INSERT INTO recent_files (path, name, opened_at)
                VALUES (?1, ?2, ?3)
                ON CONFLICT (path) DO UPDATE SET opened_at = excluded.opened_at
        "#,
    )
    .bind(&path)
    .bind(&name)
    .bind(now_unix())
    .execute(&db.0)
    .await;

    if let Err(e) = result {
        eprintln!("record_recent_file: не удалось сохранить запись: {e}");
    }

    Ok(())
}

#[tauri::command]
pub(crate) async fn list_recent_files(db: State<'_, Db>) -> Result<Vec<RecentFileEntry>, ()> {
    let rows = sqlx::query(
        r#"
            SELECT path, name, opened_at
            FROM recent_files
            ORDER BY opened_at DESC
            LIMIT ?1
        "#,
    )
    .bind(RECENT_FILES_LIMIT)
    .fetch_all(&db.0)
    .await;

    let rows = match rows {
        Ok(rows) => rows,
        Err(e) => {
            eprintln!("list_recent_files: не удалось получить список: {e}");
            return Ok(Vec::new());
        }
    };

    let entries = rows
        .into_iter()
        .map(|row| {
            let path: String = row.get("path");
            let name: String = row.get("name");
            let opened_at: i64 = row.get("opened_at");
            let location = Path::new(&path)
                .parent()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default();
            RecentFileEntry {
                path: row.get("path"),
                name,
                location,
                opened_at_label: format_timestamp(opened_at),
            }
        })
        .collect();

    Ok(entries)
}

#[tauri::command]
pub(crate) async fn clear_recent_files(db: State<'_, Db>) -> Result<(), ()> {
    if let Err(e) = sqlx::query("DELETE FROM recent_files").execute(&db.0).await {
        eprintln!("clear_recent_files: не удалось очистить список: {e}");
    }

    Ok(())
}
