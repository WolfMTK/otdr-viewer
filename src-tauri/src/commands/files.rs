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

    let mut entries: Vec<FsEntry> = read_dir.flatten().filter_map(|entry| to_fs_entry(&entry)).collect();

    sort_entries(&mut entries);
    Ok(entries)
}

fn to_fs_entry(entry: &fs::DirEntry) -> Option<FsEntry> {
    let name = entry.file_name().to_string_lossy().to_string();
    if name.starts_with(".") {
        return None;
    }

    let path = entry.path();
    let metadata = entry.metadata().ok()?;
    let is_dir = metadata.is_dir();

    if !is_dir && !is_sor_file(&path) {
        return None;
    }

    Some(FsEntry {
        name,
        path: path_to_string(&path),
        is_dir,
        size_label: (!is_dir).then(|| format_size(metadata.len())),
        modified_label: format_modified(metadata.modified()),
    })
}

fn sort_entries(entries: &mut [FsEntry]) {
    entries.sort_by_key(|e| (!e.is_dir, e.name.to_lowercase()));
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

#[cfg(test)]
mod tests {
    use std::path::Path;
    use rstest::rstest;
    use shared_types::FsEntry;
    use crate::commands::files::{format_size, is_sor_file, sort_entries};

    #[rstest]
    #[case(0, "0 Б")]
    #[case(1023, "1023 Б")]
    #[case(1024, "1 КБ")]
    #[case(1025, "2 КБ")]
    #[case(1536, "2 КБ")]
    #[case(1024 * 1024 - 1, "1024 КБ")]
    #[case(1024 * 1024, "1.0 МБ")]
    fn format_size_cases(#[case] bytes: u64, #[case] expected: &str) {
        assert_eq!(format_size(bytes), expected);
    }

    #[rstest]
    #[case("trace.sor", true)]
    #[case("TRACE.SOR", true)]
    #[case("trace.txt", false)]
    #[case("trace", false)]
    #[case(".sor", false)]
    fn is_sor_file_cases(#[case] name: &str, #[case] expected: bool) {
        assert_eq!(is_sor_file(Path::new(name)), expected);
    }

    fn entry(name: &str, is_dir: bool) -> FsEntry {
        FsEntry {
            name: name.to_string(),
            path: format!("/x/{name}"),
            is_dir,
            size_label: None,
            modified_label: None,
        }
    }

    #[test]
    fn sort_entries_dirs_first_then_case_insensitive_alpha() {
        let mut entries = vec![
            entry("b.sor", false),
            entry("Zeta", true),
            entry("A.sor", false),
            entry("alpha", true),
        ];
        sort_entries(&mut entries);
        let names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, ["alpha", "Zeta", "A.sor", "b.sor"]);
    }
}
