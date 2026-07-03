use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FsEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size_label: Option<String>,
    pub modified_label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DirListing {
    pub current_path: String,
    pub parent_path: Option<String>,
    pub entries: Vec<FsEntry>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuickLocation {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecentFileEntry {
    pub path: String,
    pub name: String,
    pub location: String,
    pub opened_at_label: String,
    pub length_label: Option<String>,
}
