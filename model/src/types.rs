use std::path::PathBuf;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub struct FsEntry {
    pub name: String,
    pub path: PathBuf,
    pub size: Option<u64>,
    pub modified: Option<SystemTime>,
}

impl FsEntry {
    pub fn is_dir(&self) -> bool {
        self.size.is_none()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecentFile {
    pub path: PathBuf,
    pub opened_at: SystemTime,
    pub length_km: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SorEvent {
    pub number: u16,
    pub distance_km: f64,
    pub loss_db: f64,
    pub refl_db: f64,
    pub kind: String,
    pub comments: String,
}

#[derive(Debug, Clone, PartialEq, Default)]
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
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SorTrace {
    pub distances_km: Vec<f64>,
    pub levels_db: Vec<f64>,
}

impl SorTrace {
    pub fn level_at(&self, distance_km: f64) -> Option<f64> {
        let distance = &self.distances_km;
        if distance.is_empty() {
            return None;
        }
        let index = distance.partition_point(|&x| x < distance_km);
        let nearest = match index {
            0 => 0,
            num if num >= distance.len() => distance.len() - 1,
            num if distance_km - distance[num - 1] <= distance[num] - distance_km => num - 1,
            num => num,
        };
        self.levels_db.get(nearest).copied()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SorData {
    pub summary: SorSummary,
    pub trace: SorTrace,
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::types::SorTrace;

    fn trace() -> SorTrace {
        SorTrace {
            distances_km: vec![0.0, 1.0, 2.0, 3.0],
            levels_db: vec![10.0, 20.0, 30.0, 40.0],
        }
    }

    #[rstest]
    #[case::nearest_left(1.4, 20.0)]
    #[case::nearest_right(1.6, 30.0)]
    #[case::tie_goes_left(1.5, 20.0)]
    #[case::before_start(-5.0, 10.0)]
    #[case::past_end(99.0, 40.0)]
    fn level_at(#[case] distance: f64, #[case] expected: f64) {
        assert_eq!(trace().level_at(distance), Some(expected));
    }

    #[test]
    fn level_at_empty_is_none() {
        assert_eq!(SorTrace::default().level_at(1.0), None);
    }
}
