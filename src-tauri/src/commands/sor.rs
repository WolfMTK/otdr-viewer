use shared_types::{SorData, SorEvent, SorSummary, SorTrace};
use sor_rs::SorFile;

use crate::commands::command_error;

const TRACE_BUCKETS: usize = 2048;

fn summarize(sor: SorFile) -> SorSummary {
    let fiber_length_km = fiber_length_km(&sor);

    let gen = sor.gen_params.as_ref();
    let fxd = sor.fxd_params.as_ref();
    let sup = sor.sup_params.as_ref();
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
        cable_id: gen.map(|g| g.cable_id.clone()),
        fiber_id: gen.map(|g| g.fiber_id.clone()),
        operator: gen.map(|g| g.operator.clone()),
        comments: gen.map(|g| g.comments.clone()),
        wavelength_nm: fxd.map(|f| f.wavelength_nm),
        otdr_supplier: sup.map(|s| s.supplier.clone()),
        otdr_model: sup.map(|s| s.otdr_name.clone()),
        otdr_serial: sup.map(|s| s.otdr_sn.clone()),
        pulse_widths_ns: fxd.map(|f| f.pulse_widths_ns.clone()).unwrap_or_default(),
        num_data_points: fxd.map(|f| f.num_data_points),
        fiber_length_km,
        total_loss_db: key_events.map(|ke| ke.summary.total_loss_db),
        orl_db: key_events.map(|ke| ke.summary.orl_db),
        events,
        error: None,
    }
}

fn fiber_length_km(sor: &SorFile) -> Option<f64> {
    sor.key_events
        .as_ref()
        .and_then(|ke| ke.end_of_fiber())
        .map(|e| e.distance_km)
        .or_else(|| {
            sor.data_points
                .as_ref()
                .and_then(|dp| dp.distances_km().last().copied())
        })
}

fn downsample_min_max(distances: &[f64], levels: &[f64], buckets: usize) -> SorTrace {
    let num = distances.len().min(levels.len());
    if num == 0 {
        return SorTrace::default();
    }

    if num <= buckets * 2 {
        return SorTrace {
            distances_km: distances[..num].to_vec(),
            levels_db: levels[..num].to_vec(),
        };
    }

    let (out_distance, out_level): (Vec<f64>, Vec<f64>) = (0..buckets)
        .flat_map(|bucket| {
            let start = bucket * num / buckets;
            let end = ((bucket + 1) * num / buckets).max(start + 1);

            let (i_min, i_max) = (start..end).fold((start, start), |(i_min, i_max), i| {
                let i_min = if levels[i] < levels[i_min] { i } else { i_min };
                let i_max = if levels[i] > levels[i_max] { i } else { i_max };
                (i_min, i_max)
            });

            let (first, second) = if i_min <= i_max { (i_min, i_max) } else { (i_max, i_min) };

            std::iter::once(first)
                .chain((second != first).then_some(second))
                .map(|i| (distances[i], levels[i]))
        })
        .unzip();

    SorTrace {
        distances_km: out_distance,
        levels_db: out_level,
    }
}

pub(crate) fn read_fiber_length_km(path: &str) -> Option<f64> {
    let sor = SorFile::from_file(path, false).ok()?;
    fiber_length_km(&sor)
}

#[tauri::command]
pub(crate) fn parse_sor_file(path: String) -> Result<SorData, String> {
    SorFile::from_file(&path, false)
        .map(|sor| {
            let trace = sor
                .data_points
                .as_ref()
                .map(|dp| downsample_min_max(&dp.distances_km(), &dp.levels_db(), TRACE_BUCKETS))
                .unwrap_or_default();
            SorData {
                summary: summarize(sor),
                trace,
            }
        })
        .map_err(|e| command_error(&format!("parse_sor_file: {path}"), "Не удалось прочитать .sor-файл", e))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::commands::sor::{downsample_min_max, TRACE_BUCKETS};

    fn ramp(num: usize) -> (Vec<f64>, Vec<f64>) {
        let distance: Vec<f64> = (0..num).map(|i| i as f64 * 0.001).collect();
        let level: Vec<f64> = (0..num).map(|i| -(i as f64) * 0.01).collect();
        (distance, level)
    }

    #[rstest]
    fn downsample_passes_short_traces_through() {
        let (distance, level) = ramp(100);
        let trace = downsample_min_max(&distance, &level, 2048);
        assert_eq!(trace.distances_km.len(), 100);
        assert_eq!(trace.levels_db, level);
    }

    #[rstest]
    fn downsample_caps_point_count() {
        let (distance, level) = ramp(30_000);
        let trace = downsample_min_max(&distance, &level, TRACE_BUCKETS);
        assert!(trace.distances_km.len() <= 4096);
        assert_eq!(trace.distances_km.len(), trace.levels_db.len());
    }

    #[rstest]
    fn downsample_preserves_narrow_spike() {
        let (distance, mut level) = ramp(30_000);
        level[12_345] = 5.0;
        let t = downsample_min_max(&distance, &level, 2048);
        assert!(t.levels_db.contains(&5.0));
    }

    #[rstest]
    #[case(0)]
    #[case(1)]
    #[case(2)]
    fn downsample_handles_tiny_inputs(#[case] num: usize) {
        let (distance, level) = ramp(num);
        let trace = downsample_min_max(&distance, &level, 2048);
        assert_eq!(trace.distances_km.len(), num);
    }
}
