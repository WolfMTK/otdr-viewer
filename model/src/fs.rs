use std::path::Path;
use std::{fs, io};

use crate::types::FsEntry;
const SOR_EXTENSION: &str = "sor";

pub fn is_sor_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case(SOR_EXTENSION))
}

pub fn list_directory(dir: &Path) -> io::Result<Vec<FsEntry>> {
    let mut entries: Vec<FsEntry> = fs::read_dir(dir)?
        .flatten()
        .filter_map(|entry| to_fs_entry(&entry))
        .collect();
    entries.sort_by_cached_key(|e| (!e.is_dir(), e.name.to_lowercase()));
    Ok(entries)
}

fn to_fs_entry(entry: &fs::DirEntry) -> Option<FsEntry> {
    let name = entry.file_name().to_string_lossy().into_owned();
    if name.starts_with('.') {
        return None;
    }

    let path = entry.path();
    let metadata = fs::metadata(&path).ok()?;
    let is_dir = metadata.is_dir();
    if !is_dir && !is_sor_file(&path) {
        return None;
    }

    Some(FsEntry {
        name,
        path,
        size: (!is_dir).then_some(metadata.len()),
        modified: metadata.modified().ok(),
    })
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use rstest::rstest;
    use tempfile::TempDir;

    use crate::fs::{is_sor_file, list_directory};

    #[rstest]
    #[case("trace.sor", true)]
    #[case("TRACE.SOR", true)]
    #[case("trace.txt", false)]
    #[case("trace", false)]
    fn is_sor_file_cases(#[case] name: &str, #[case] expected: bool) {
        assert_eq!(is_sor_file(Path::new(name)), expected);
    }

    fn names(dir: &Path) -> Vec<String> {
        list_directory(dir)
            .unwrap()
            .into_iter()
            .map(|e| e.name)
            .collect()
    }

    #[test]
    fn list_directory_filters_and_sorts() {
        let dir = TempDir::new().unwrap();
        for sub in ["Zeta", "alpha", ".hidden"] {
            fs::create_dir(dir.path().join(sub)).unwrap();
        }
        for file in ["b.sor", "A.SOR", "notes.txt", ".c.sor"] {
            fs::write(dir.path().join(file), b"x").unwrap();
        }

        assert_eq!(names(dir.path()), ["alpha", "Zeta", "A.SOR", "b.sor"]);
    }

    #[cfg(unix)]
    #[test]
    fn list_directory_follows_symlinks() {
        use std::os::unix::fs::symlink;

        let dir = TempDir::new().unwrap();
        let root = dir.path();
        fs::create_dir(root.join("real_dir")).unwrap();
        fs::write(root.join("real.sor"), [0u8; 100]).unwrap();
        symlink(root.join("real_dir"), root.join("link_dir")).unwrap();
        symlink(root.join("real.sor"), root.join("link.sor")).unwrap();
        symlink(root.join("missing"), root.join("broken.sor")).unwrap();

        let entries = list_directory(root).unwrap();
        let link_dir = entries.iter().find(|e| e.name == "link_dir").unwrap();
        let link_sor = entries.iter().find(|e| e.name == "link.sor").unwrap();

        assert!(link_dir.is_dir());
        assert_eq!(link_sor.size, Some(100));
        assert!(entries.iter().all(|e| e.name != "broken.sor"));
    }
}
