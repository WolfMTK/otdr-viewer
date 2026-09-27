use std::path::{Path, PathBuf};

use model::fs::is_sor_file;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropHint {
    Supported,
    Unsupported,
}

pub fn drop_hint<'a>(paths: impl IntoIterator<Item = Option<&'a Path>>) -> Option<DropHint> {
    let mut any = false;
    for path in paths {
        any = true;
        if path.is_none_or(is_sor_file) {
            return Some(DropHint::Supported);
        }
    }
    any.then_some(DropHint::Unsupported)
}

pub fn first_sor_file<'a>(paths: impl IntoIterator<Item = &'a Path>) -> Option<PathBuf> {
    paths
        .into_iter()
        .find(|path| is_sor_file(path))
        .map(Path::to_path_buf)
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use rstest::rstest;

    use crate::drop::{DropHint, drop_hint, first_sor_file};

    #[rstest]
    #[case::nothing(&[], None)]
    #[case::sor(&[Some("a.sor")], Some(DropHint::Supported))]
    #[case::other(&[Some("a.txt")], Some(DropHint::Unsupported))]
    #[case::mixed(&[Some("a.txt"), Some("b.SOR")], Some(DropHint::Supported))]
    #[case::unknown_name(&[None], Some(DropHint::Supported))]
    fn hint(#[case] paths: &[Option<&str>], #[case] expected: Option<DropHint>) {
        let paths = paths.iter().map(|p| p.map(Path::new));
        assert_eq!(drop_hint(paths), expected);
    }

    #[test]
    fn first_sor_file_skips_other_files() {
        let paths = [
            Path::new("notes.txt"),
            Path::new("b.sor"),
            Path::new("c.sor"),
        ];
        assert_eq!(first_sor_file(paths), Some(PathBuf::from("b.sor")));
    }

    #[test]
    fn no_sor_file_opens_nothing() {
        assert_eq!(first_sor_file([Path::new("notes.txt")]), None);
    }
}
