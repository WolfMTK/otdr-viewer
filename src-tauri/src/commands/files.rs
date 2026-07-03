use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Local, Utc};
use shared_types::{DirListing, FsEntry, QuickLocation};
use tauri::{Manager, Window};

const KB: f64 = 1024.0;
const MB: f64 = KB * KB;
const SOR_EXTENSION: &str = "sor";

fn path_to_string(path: &Path) -> String {
    path.to_string_lossy().to_string()
}

fn format_size(bytes: u64) -> String {
    let bytes_f = bytes as f64;

    if bytes_f >= MB {
        format!("{:.1} МБ", bytes_f / MB)
    } else if bytes_f >= KB {
        format!("{:.0} КБ", (bytes_f / KB).ceil().max(1.0))
    } else {
        format!("{bytes} Б")
    }
}

fn format_modified(modified: std::io::Result<SystemTime>) -> Option<String> {
    let utc: DateTime<Utc> = modified.ok()?.into();
    Some(utc.with_timezone(&Local).format("%d.%m.%Y").to_string())
}

fn is_sor_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case(SOR_EXTENSION))
}

fn read_entries(dir: &Path) -> Result<Vec<FsEntry>, String> {
    let read_dir = fs::read_dir(dir).map_err(|e| format!("Не удалось прочитать папку: {e}"))?;

    let mut dirs = Vec::new();
    let mut files = Vec::new();

    for entry in read_dir.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        if name.starts_with('.') {
            continue;
        }

        let Ok(metadata) = entry.metadata() else { continue };
        let modified_label = format_modified(metadata.modified());

        if metadata.is_dir() {
            dirs.push(FsEntry {
                name,
                path: path_to_string(&path),
                is_dir: true,
                size_label: None,
                modified_label,
            });
        } else if is_sor_file(&path) {
            files.push(FsEntry {
                name,
                path: path_to_string(&path),
                is_dir: false,
                size_label: Some(format_size(metadata.len())),
                modified_label,
            });
        }
    }

    dirs.sort_by_key(|e| e.name.to_lowercase());
    files.sort_by_key(|e| e.name.to_lowercase());
    dirs.extend(files);

    Ok(dirs)
}

#[tauri::command]
pub(crate) fn list_directory(window: Window, path: Option<String>) -> DirListing {
    let dir = path
        .map(PathBuf::from)
        .unwrap_or_else(|| window.path().home_dir().unwrap_or_else(|_| PathBuf::from(".")));

    let current_path = path_to_string(&dir);
    let parent_path = dir.parent().map(path_to_string);

    let (entries, error) = if !dir.is_dir() {
        (Vec::new(), Some("Указанный путь не является папкой".to_string()))
    } else {
        match read_entries(&dir) {
            Ok(entries) => (entries, None),
            Err(e) => (Vec::new(), Some(e)),
        }
    };

    DirListing {
        current_path,
        parent_path,
        entries,
        error,
    }
}

#[tauri::command]
pub(crate) fn list_quick_locations(window: Window) -> Vec<QuickLocation> {
    let resolver = window.path();

    let candidates = [
        ("Домашняя папка", resolver.home_dir()),
        ("Рабочий стол", resolver.desktop_dir()),
        ("Документы", resolver.document_dir()),
        ("Загрузки", resolver.download_dir()),
    ];

    candidates
        .into_iter()
        .filter_map(|(name, path)| path.ok().map(|p| (name, p)))
        .filter(|(_, path)| path.is_dir())
        .map(|(name, path)| QuickLocation {
            name: name.to_string(),
            path: path_to_string(&path),
        })
        .collect()
}
