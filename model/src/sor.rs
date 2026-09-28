use std::path::Path;

pub use sor_rs::SorError;
use sor_rs::SorFile;

use crate::types::{SorData, SorEvent, SorSummary, SorTrace};

const NO_DATA: u16 = u16::MAX;

pub fn load(path: &Path) -> Result<SorData, SorError> {
    let sor = SorFile::from_file(path, false)?;
    let trace = sor
        .data_points
        .as_ref()
        .map(|dp| without_missing(&dp.distances_km(), &dp.levels_db(), &dp.raw_data))
        .unwrap_or_default();

    let fiber_length_km = end_of_fiber_km(&sor).or(trace.distances_km.last().copied());
    Ok(SorData {
        summary: summarize(&sor, fiber_length_km),
        trace,
    })
}

fn without_missing(distances: &[f64], levels: &[f64], raw: &[u16]) -> SorTrace {
    let (distances_km, levels_db) = distances
        .iter()
        .zip(levels)
        .zip(raw)
        .filter(|(_, raw)| **raw != NO_DATA)
        .map(|((&d, &l), _)| (d, l))
        .unzip();
    SorTrace {
        distances_km,
        levels_db,
    }
}

pub fn read_fiber_length_km(path: &Path) -> Option<f64> {
    let sor = SorFile::from_file(path, false).ok()?;
    end_of_fiber_km(&sor).or_else(|| {
        sor.data_points
            .as_ref()
            .and_then(|dp| dp.distances_km().last().copied())
    })
}

fn end_of_fiber_km(sor: &SorFile) -> Option<f64> {
    sor.key_events
        .as_ref()
        .and_then(|ke| ke.end_of_fiber())
        .map(|e| e.distance_km)
}

fn summarize(sor: &SorFile, fiber_length_km: Option<f64>) -> SorSummary {
    let general = sor.gen_params.as_ref();
    let fixed = sor.fxd_params.as_ref();
    let supplier = sor.sup_params.as_ref();
    let key_events = sor.key_events.as_ref();

    let events = key_events
        .map(|ke| {
            ke.events
                .iter()
                .map(|e| SorEvent {
                    number: e.number,
                    distance_km: e.distance_km,
                    loss_db: e.loss_db,
                    refl_db: e.refl_db,
                    kind: e.subtype_str().to_string(),
                    comments: e.comments.clone(),
                })
                .collect()
        })
        .unwrap_or_default();

    SorSummary {
        cable_id: general.map(|g| g.cable_id.clone()),
        fiber_id: general.map(|g| g.fiber_id.clone()),
        operator: general.map(|g| g.operator.clone()),
        comments: general.map(|g| g.comments.clone()),
        wavelength_nm: fixed.map(|f| f.wavelength_nm),
        otdr_supplier: supplier.map(|s| s.supplier.clone()),
        otdr_model: supplier.map(|s| s.otdr_name.clone()),
        otdr_serial: supplier.map(|s| s.otdr_sn.clone()),
        pulse_widths_ns: fixed.map(|f| f.pulse_widths_ns.clone()).unwrap_or_default(),
        num_data_points: fixed.map(|f| f.num_data_points),
        fiber_length_km,
        total_loss_db: key_events.map(|ke| ke.summary.total_loss_db),
        orl_db: key_events.map(|ke| ke.summary.orl_db),
        events,
    }
}

pub fn downsample_min_max(distances: &[f64], levels: &[f64], buckets: usize) -> SorTrace {
    let num = distances.len().min(levels.len());
    if num <= buckets * 2 {
        return SorTrace {
            distances_km: distances[..num].to_vec(),
            levels_db: levels[..num].to_vec(),
        };
    }

    let (distances_km, levels_db) = (0..buckets)
        .flat_map(|bucket| {
            let start = bucket * num / buckets;
            let end = ((bucket + 1) * num / buckets).max(start + 1);

            let (i_min, i_max) = (start..end).fold((start, start), |(i_min, i_max), i| {
                let i_min = if levels[i] < levels[i_min] { i } else { i_min };
                let i_max = if levels[i] > levels[i_max] { i } else { i_max };
                (i_min, i_max)
            });

            let (first, second) = (i_min.min(i_max), i_min.max(i_max));
            std::iter::once(first)
                .chain((second != first).then_some(second))
                .map(|i| (distances[i], levels[i]))
        })
        .unzip();

    SorTrace {
        distances_km,
        levels_db,
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::sor::{NO_DATA, downsample_min_max, without_missing};

    const LONG: usize = 30_000;
    const BUCKETS: usize = 2048;

    fn ramp(num: usize) -> (Vec<f64>, Vec<f64>) {
        let distance = (0..num).map(|i| i as f64 * 0.001).collect();
        let level = (0..num).map(|i| -(i as f64) * 0.01).collect();
        (distance, level)
    }

    #[rstest]
    #[case::empty(0)]
    #[case::short(100)]
    fn downsample_passes_short_traces_through(#[case] num: usize) {
        let (distance, level) = ramp(num);
        let trace = downsample_min_max(&distance, &level, BUCKETS);
        assert_eq!(trace.distances_km, distance);
        assert_eq!(trace.levels_db, level);
    }

    #[test]
    fn downsample_caps_point_count() {
        let (distance, level) = ramp(LONG);
        let trace = downsample_min_max(&distance, &level, BUCKETS);
        assert!(trace.distances_km.len() <= 2 * BUCKETS);
        assert_eq!(trace.distances_km.len(), trace.levels_db.len());
    }

    #[rstest]
    #[case::peak(5.0)]
    #[case::dip(-1000.0)]
    fn downsample_preserves_narrow_extremes(#[case] value: f64) {
        let (distance, mut level) = ramp(LONG);
        level[12_345] = value;
        let trace = downsample_min_max(&distance, &level, BUCKETS);
        assert!(trace.levels_db.contains(&value));
    }

    #[test]
    fn downsample_keeps_distances_sorted() {
        let (distance, mut level) = ramp(LONG);
        for (i, l) in level.iter_mut().enumerate() {
            *l = if i % 7 == 0 { 10.0 } else { -(i as f64) };
        }
        let trace = downsample_min_max(&distance, &level, BUCKETS);
        assert!(trace.distances_km.is_sorted());
    }

    #[test]
    fn missing_samples_are_dropped() {
        let distances = [0.0, 1.0, 2.0];
        let levels = [10.0, 65.535, 12.0];
        let trace = without_missing(&distances, &levels, &[10_000, NO_DATA, 12_000]);
        assert_eq!(trace.distances_km, [0.0, 2.0]);
        assert_eq!(trace.levels_db, [10.0, 12.0]);
    }
}
