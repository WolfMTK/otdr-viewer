use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::thread;

use model::fs::is_sor_file;
use model::types::{SorData, SorTrace};

use crate::chart_view::{ChartView, DistanceRange, LevelRange};
use crate::events::{EventRow, event_rows};
use crate::markers::{Markers, ab_endpoints};
use crate::params::{Params, params};

type LoadResult = Result<SorData, String>;

enum Progress {
    Picked(PathBuf),
    Loaded(LoadResult),
    Cancelled,
}

#[derive(Debug)]
pub struct OpenedFile {
    path: PathBuf,
    data: SorData,
    chart: ChartView,
    markers: Markers,
    events: Vec<EventRow>,
    params: Params,
}

pub struct ChartParts<'a> {
    pub trace: &'a SorTrace,
    pub view: &'a mut ChartView,
    pub markers: &'a mut Markers,
}

impl OpenedFile {
    fn new(path: PathBuf, data: SorData) -> Self {
        let trace = &data.trace;
        let chart = ChartView::new(full_range(trace), LevelRange::around(&trace.levels_db));
        let markers = ab_endpoints(&data.summary.events, trace);
        let events = event_rows(&data.summary.events, trace);
        let params = params(&data.summary);
        Self {
            path,
            data,
            chart,
            markers,
            events,
            params,
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn data(&self) -> &SorData {
        &self.data
    }

    pub fn events(&self) -> &[EventRow] {
        &self.events
    }

    pub fn params(&self) -> &Params {
        &self.params
    }

    pub fn chart_mut(&mut self) -> &mut ChartView {
        &mut self.chart
    }

    pub fn chart_parts_mut(&mut self) -> ChartParts<'_> {
        ChartParts {
            trace: &self.data.trace,
            view: &mut self.chart,
            markers: &mut self.markers,
        }
    }
}

fn full_range(trace: &SorTrace) -> DistanceRange {
    match (trace.distances_km.first(), trace.distances_km.last()) {
        (Some(&first), Some(&last)) if last > first => DistanceRange::new(first, last),
        (Some(&first), _) => DistanceRange::new(first, first + 1.0),
        _ => DistanceRange::new(0.0, 1.0),
    }
}

#[derive(Debug, Default)]
pub enum DocumentState {
    #[default]
    Empty,
    Loading(PathBuf),
    Opened(Box<OpenedFile>),
    Failed {
        path: PathBuf,
        message: String,
    },
}

#[derive(Debug, Default)]
pub struct DocumentViewModel {
    state: DocumentState,
    pending: Option<Receiver<Progress>>,
    picking: bool,
}

impl DocumentViewModel {
    pub fn state(&self) -> &DocumentState {
        &self.state
    }

    pub fn opened(&self) -> Option<&OpenedFile> {
        match &self.state {
            DocumentState::Opened(file) => Some(file),
            _ => None,
        }
    }

    pub fn opened_mut(&mut self) -> Option<&mut OpenedFile> {
        match &mut self.state {
            DocumentState::Opened(file) => Some(file),
            _ => None,
        }
    }

    pub fn is_picking(&self) -> bool {
        self.picking
    }

    pub fn open(&mut self, path: PathBuf, on_update: impl Fn() + Send + 'static) {
        self.start(move || Some(path), load_sor, on_update, false);
    }

    pub fn pick_and_open(
        &mut self,
        pick: impl FnOnce() -> Option<PathBuf> + Send + 'static,
        on_update: impl Fn() + Send + 'static,
    ) {
        if !self.picking {
            self.start(pick, load_sor, on_update, true);
        }
    }

    fn start(
        &mut self,
        pick: impl FnOnce() -> Option<PathBuf> + Send + 'static,
        load: impl FnOnce(&Path) -> LoadResult + Send + 'static,
        on_update: impl Fn() + Send + 'static,
        picking: bool,
    ) {
        let (sender, receiver) = mpsc::channel();
        thread::spawn(move || {
            let progress = match pick() {
                None => Progress::Cancelled,
                Some(path) => {
                    if sender.send(Progress::Picked(path.clone())).is_err() {
                        return;
                    }
                    on_update();
                    if is_sor_file(&path) {
                        Progress::Loaded(load(&path))
                    } else {
                        Progress::Loaded(Err("поддерживаются только файлы .sor".to_string()))
                    }
                }
            };
            let _ = sender.send(progress);
            on_update();
        });
        self.pending = Some(receiver);
        self.picking = picking;
    }

    pub fn poll(&mut self) -> bool {
        let Some(receiver) = self.pending.take() else {
            return false;
        };
        loop {
            match receiver.try_recv() {
                Ok(Progress::Picked(path)) => {
                    self.picking = false;
                    self.state = DocumentState::Loading(path);
                }
                Ok(Progress::Loaded(result)) => return self.finish(result),
                Ok(Progress::Cancelled) => {
                    self.picking = false;
                    return false;
                }
                Err(TryRecvError::Empty) => {
                    self.pending = Some(receiver);
                    return false;
                }
                Err(TryRecvError::Disconnected) => {
                    self.picking = false;
                    return self.finish(Err("загрузка прервана".to_string()));
                }
            }
        }
    }

    fn finish(&mut self, result: LoadResult) -> bool {
        let path = match std::mem::take(&mut self.state) {
            DocumentState::Loading(path) => path,
            other => {
                self.state = other;
                return false;
            }
        };
        match result {
            Ok(data) => {
                self.state = DocumentState::Opened(Box::new(OpenedFile::new(path, data)));
                true
            }
            Err(error) => {
                self.state = DocumentState::Failed {
                    path,
                    message: format!("Не удалось открыть файл: {error}"),
                };
                false
            }
        }
    }
}

fn load_sor(path: &Path) -> LoadResult {
    model::sor::load(path).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, mpsc};
    use std::time::{Duration, Instant};

    use model::types::{SorData, SorTrace};

    use crate::document::{DocumentState, DocumentViewModel, LoadResult};

    fn data(last_km: f64) -> SorData {
        SorData {
            summary: Default::default(),
            trace: SorTrace {
                distances_km: vec![0.0, last_km],
                levels_db: vec![0.0, 1.0],
            },
        }
    }

    fn file(name: &str) -> impl FnOnce() -> Option<PathBuf> + Send + 'static {
        let path = PathBuf::from(name);
        move || Some(path)
    }

    fn ok(last_km: f64) -> impl FnOnce(&Path) -> LoadResult + Send + 'static {
        move |_| Ok(data(last_km))
    }

    fn wait_until_idle(vm: &mut DocumentViewModel) {
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            vm.poll();
            if vm.pending.is_none() {
                return;
            }
            assert!(Instant::now() < deadline, "worker did not finish");
            std::thread::sleep(Duration::from_millis(1));
        }
    }

    fn opened_path(vm: &mut DocumentViewModel) -> PathBuf {
        vm.opened_mut()
            .expect("file should be opened")
            .path()
            .to_path_buf()
    }

    #[test]
    fn successful_load_opens_file_and_notifies() {
        let mut vm = DocumentViewModel::default();
        let notified = Arc::new(AtomicBool::new(false));
        let flag = notified.clone();

        vm.start(file("a.sor"), ok(10.0), move || flag.store(true, Ordering::SeqCst), false);
        wait_until_idle(&mut vm);

        assert_eq!(opened_path(&mut vm), PathBuf::from("a.sor"));
        assert_eq!(vm.opened_mut().unwrap().chart_mut().full().end(), 10.0);
        assert!(notified.load(Ordering::SeqCst));
    }

    #[test]
    fn failed_load_keeps_path_and_reason() {
        let mut vm = DocumentViewModel::default();
        vm.start(file("bad.sor"), |_| Err("broken header".into()), || {}, false);
        wait_until_idle(&mut vm);

        let DocumentState::Failed { path, message } = vm.state() else {
            panic!("expected failure, got {:?}", vm.state());
        };
        assert_eq!(path, &PathBuf::from("bad.sor"));
        assert!(message.contains("broken header"), "{message}");
    }

    #[test]
    fn non_sor_file_is_rejected_without_loading() {
        let mut vm = DocumentViewModel::default();
        vm.start(file("notes.txt"), |_| panic!("must not load"), || {}, false);
        wait_until_idle(&mut vm);
        assert!(matches!(vm.state(), DocumentState::Failed { .. }));
    }

    #[test]
    fn newer_open_wins_over_slow_previous_one() {
        let mut vm = DocumentViewModel::default();
        let (release_slow, gate) = mpsc::channel::<()>();

        let slow = move |_: &Path| -> LoadResult {
            gate.recv().ok();
            Ok(data(1.0))
        };
        vm.start(file("slow.sor"), slow, || {}, false);
        vm.start(file("fast.sor"), ok(2.0), || {}, false);
        wait_until_idle(&mut vm);
        release_slow.send(()).unwrap();

        assert_eq!(opened_path(&mut vm), PathBuf::from("fast.sor"));
    }

    #[test]
    fn cancelled_dialog_keeps_opened_file() {
        let mut vm = DocumentViewModel::default();
        vm.start(file("a.sor"), ok(1.0), || {}, false);
        wait_until_idle(&mut vm);

        vm.start(|| None, |_| panic!("must not load"), || {}, true);
        wait_until_idle(&mut vm);

        assert_eq!(opened_path(&mut vm), PathBuf::from("a.sor"));
        assert!(!vm.is_picking());
    }

    #[test]
    fn second_dialog_is_ignored_while_first_is_open() {
        let mut vm = DocumentViewModel::default();
        let (close_dialog, dialog) = mpsc::channel::<Option<PathBuf>>();

        let second_opened = Arc::new(AtomicBool::new(false));
        let flag = second_opened.clone();

        vm.pick_and_open(move || dialog.recv().ok().flatten(), || {});
        assert!(vm.is_picking());
        vm.pick_and_open(
            move || {
                flag.store(true, Ordering::SeqCst);
                None
            },
            || {},
        );

        close_dialog.send(None).unwrap();
        wait_until_idle(&mut vm);
        assert!(!vm.is_picking());
        assert!(!second_opened.load(Ordering::SeqCst));
    }

    #[test]
    fn single_point_trace_gets_non_empty_range() {
        let mut vm = DocumentViewModel::default();
        let single_point = |_: &Path| -> LoadResult {
            Ok(SorData {
                summary: Default::default(),
                trace: SorTrace {
                    distances_km: vec![3.0],
                    levels_db: vec![5.0],
                },
            })
        };
        vm.start(file("dot.sor"), single_point, || {}, false);
        wait_until_idle(&mut vm);

        let full = vm.opened_mut().unwrap().chart_mut().full();
        assert!(full.span() > 0.0);
    }

    #[test]
    fn poll_reports_a_newly_opened_file_once() {
        let mut vm = DocumentViewModel::default();
        vm.start(file("a.sor"), ok(1.0), || {}, false);

        let deadline = Instant::now() + Duration::from_secs(5);
        while !vm.poll() {
            assert!(Instant::now() < deadline, "file was not opened");
            std::thread::sleep(Duration::from_millis(1));
        }
        assert!(!vm.poll());
    }
}
