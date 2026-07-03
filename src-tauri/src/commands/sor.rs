use serde::Serialize;
use sor_rs::SorFile;

#[derive(Serialize, Clone)]
pub(crate) struct SorEvent {
    number: u16,
    distance_km: f64,
    loss_db: f64,
    refl_db: f64,
    kind: &'static str,
    comments: String,
}

#[derive(Serialize, Clone, Default)]
pub(crate) struct SorSummary {
    cable_id: Option<String>,
    fiber_id: Option<String>,
    operator: Option<String>,
    comments: Option<String>,
    wavelength_nm: Option<f64>,
    otdr_supplier: Option<String>,
    otdr_model: Option<String>,
    otdr_serial: Option<String>,
    pulse_widths_ns: Vec<u16>,
    num_data_points: Option<u32>,
    fiber_length_km: Option<f64>,
    total_loss_db: Option<f64>,
    orl_db: Option<f64>,
    events: Vec<SorEvent>,
    error: Option<String>,
}

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
                    kind: e.subtype_str(),
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

pub(crate) fn read_fiber_length_km(path: &str) -> Option<f64> {
    let sor = SorFile::from_file(path, false).ok()?;
    fiber_length_km(&sor)
}

#[tauri::command]
pub(crate) fn parse_sor_file(path: String) -> SorSummary {
    match SorFile::from_file(&path, false) {
        Ok(sor) => summarize(sor),
        Err(e) => SorSummary {
            error: Some(e.to_string()),
            ..Default::default()
        },
    }
}
