use std::path::{Path, PathBuf};
use std::time::SystemTime;

use model::types::RecentFile;

use crate::format::{format_date, format_length_km};

const LIMIT: usize = 20;

#[derive(Debug, Clone, PartialEq)]
pub struct RecentFileRow {
    pub path: PathBuf,
    pub name: String,
    pub location: String,
    pub opened_at: String,
    pub length: Option<String>,
}

#[derive(Debug)]
pub struct RecentFilesViewModel {
    files: Vec<RecentFile>,
    pub query: String,
    pub panel_open: bool,
}

impl Default for RecentFilesViewModel {
    fn default() -> Self {
        Self {
            files: Vec::new(),
            query: String::new(),
            panel_open: true,
        }
    }
}

impl RecentFilesViewModel {
    pub fn files(&self) -> &[RecentFile] {
        &self.files
    }

    pub fn restore(&mut self, mut files: Vec<RecentFile>) {
        files.truncate(LIMIT);
        self.files = files;
    }

    pub fn record(&mut self, path: &Path, length_km: Option<f64>, opened_at: SystemTime) {
        self.files.retain(|f| f.path != path);
        self.files.insert(
            0,
            RecentFile {
                path: path.to_path_buf(),
                opened_at,
                length_km,
            },
        );
        self.files.truncate(LIMIT);
    }

    pub fn clear(&mut self) {
        self.files.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    pub fn rows(&self) -> Vec<RecentFileRow> {
        filter_by_name(&self.files, &self.query)
            .into_iter()
            .map(|file| RecentFileRow {
                path: file.path.clone(),
                name: file_name(&file.path),
                location: file
                    .path
                    .parent()
                    .map(|p| p.display().to_string())
                    .unwrap_or_default(),
                opened_at: format_date(file.opened_at),
                length: file.length_km.map(format_length_km),
            })
            .collect()
    }
}

pub fn filter_by_name<'a>(files: &'a [RecentFile], query: &str) -> Vec<&'a RecentFile> {
    let query = query.trim().to_lowercase();
    files
        .iter()
        .filter(|f| query.is_empty() || file_name(&f.path).to_lowercase().contains(&query))
        .collect()
}

pub fn file_name(path: &Path) -> String {
    path.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string())
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use std::time::{Duration, SystemTime};

    use model::types::RecentFile;
    use rstest::rstest;

    use crate::recent_files::{LIMIT, RecentFilesViewModel, filter_by_name};

    fn at(seconds: u64) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_secs(seconds)
    }

    fn file(path: &str) -> RecentFile {
        RecentFile {
            path: PathBuf::from(path),
            opened_at: at(0),
            length_km: None,
        }
    }

    fn paths(vm: &RecentFilesViewModel) -> Vec<&Path> {
        vm.files().iter().map(|f| f.path.as_path()).collect()
    }

    #[test]
    fn newest_file_goes_first() {
        let mut vm = RecentFilesViewModel::default();
        vm.record(Path::new("/a.sor"), None, at(1));
        vm.record(Path::new("/b.sor"), None, at(2));
        assert_eq!(paths(&vm), [Path::new("/b.sor"), Path::new("/a.sor")]);
    }

    #[test]
    fn reopening_moves_file_to_top_without_duplicate() {
        let mut vm = RecentFilesViewModel::default();
        vm.record(Path::new("/a.sor"), None, at(1));
        vm.record(Path::new("/b.sor"), None, at(2));
        vm.record(Path::new("/a.sor"), Some(5.0), at(3));

        assert_eq!(paths(&vm), [Path::new("/a.sor"), Path::new("/b.sor")]);
        assert_eq!(vm.files()[0].length_km, Some(5.0));
        assert_eq!(vm.files()[0].opened_at, at(3));
    }

    #[test]
    fn list_is_capped() {
        let mut vm = RecentFilesViewModel::default();
        for i in 0..LIMIT + 5 {
            vm.record(Path::new(&format!("/{i}.sor")), None, at(i as u64));
        }
        assert_eq!(vm.files().len(), LIMIT);
        assert_eq!(vm.files()[0].path, PathBuf::from(format!("/{}.sor", LIMIT + 4)));
    }

    #[test]
    fn clear_empties_the_list() {
        let mut vm = RecentFilesViewModel::default();
        vm.record(Path::new("/a.sor"), None, at(1));
        vm.clear();
        assert!(vm.is_empty());
    }

    #[rstest]
    #[case::empty_query("", 3)]
    #[case::case_insensitive("TRACE", 2)]
    #[case::trimmed("  east ", 1)]
    #[case::folder_is_not_searched("cables", 0)]
    fn filter(#[case] query: &str, #[case] expected: usize) {
        let files = [
            file("/cables/trace-east.sor"),
            file("/cables/trace-west.sor"),
            file("/cables/other.sor"),
        ];
        assert_eq!(filter_by_name(&files, query).len(), expected);
    }

    #[test]
    fn rows_split_name_and_folder() {
        let mut vm = RecentFilesViewModel::default();
        vm.record(Path::new("/data/line.sor"), Some(12.34), at(1));
        let row = &vm.rows()[0];
        assert_eq!(row.name, "line.sor");
        assert_eq!(row.location, "/data");
        assert_eq!(row.length.as_deref(), Some("12.3 км"));
    }
}
