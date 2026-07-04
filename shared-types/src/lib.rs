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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SorEvent {
    pub number: u16,
    pub distance_km: f64,
    pub loss_db: f64,
    pub refl_db: f64,
    pub kind: String,
    pub comments: String,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SorSummary {
    pub cable_id: Option<String>,
    pub fiber_id: Option<String>,
    pub operator: Option<String>,
    pub comments: Option<String>,
    pub wavelength_nm: Option<f64>,
    pub otdr_supplier: Option<String>,
    pub otdr_model: Option<String>,
    pub otdr_serial: Option<String>,
    pub pulse_widths_ns: Vec<u16>,
    pub num_data_points: Option<u32>,
    pub fiber_length_km: Option<f64>,
    pub total_loss_db: Option<f64>,
    pub orl_db: Option<f64>,
    pub events: Vec<SorEvent>,
    pub error: Option<String>,
}
