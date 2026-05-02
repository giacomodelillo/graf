use std::sync::Arc;
use std::sync::RwLock;

use fdg_sim::petgraph::graph::NodeIndex;

use crate::config::GrafConfig;
use crate::graph::input::GraphMouseState;
use crate::linker::FileData;

pub struct AppState {
    pub graph_state: Option<Arc<RwLock<crate::graph::GraphState>>>,
    pub graph_kill_tx: Option<std::sync::mpsc::Sender<()>>,
    pub graph_mouse_state: GraphMouseState,
    pub files: Vec<FileData>,
    pub show_help: bool,
    pub config_errors: Vec<String>,
    pub search_active: bool,
    pub search_query: String,
    pub search_results: Vec<(NodeIndex, String)>,
    pub search_selected: usize,
    pub search_cursor: usize,
    pub show_minimap: bool,
    pub show_legend: bool,
    pub show_grid: bool,
    pub show_status_bar: bool,
}

impl AppState {
    pub fn new(config: &GrafConfig, files: Vec<FileData>, config_errors: Vec<String>) -> Self {
        let graph_state = crate::graph::GraphState::new(&files, config);
        let state = Arc::new(RwLock::new(graph_state));
        let (kill_tx, kill_rx) = std::sync::mpsc::channel();
        crate::graph::physics::start_physics(state.clone(), config, kill_rx);

        Self {
            graph_state: Some(state),
            graph_kill_tx: Some(kill_tx),
            graph_mouse_state: GraphMouseState::default(),
            files,
            show_help: false,
            config_errors,
            search_active: false,
            search_query: String::new(),
            search_results: Vec::new(),
            search_selected: 0,
            search_cursor: 0,
            show_minimap: config.visual.show_minimap,
            show_legend: config.visual.show_legend,
            show_grid: config.visual.show_grid,
            show_status_bar: config.display.show_status_bar,
        }
    }

    pub fn refresh_simulation(&mut self, config: &GrafConfig) {
        if let Some(kill_tx) = self.graph_kill_tx.take() {
            let _ = kill_tx.send(());
        }
        let graph_state = crate::graph::GraphState::new(&self.files, config);
        let state = Arc::new(RwLock::new(graph_state));
        let (kill_tx, kill_rx) = std::sync::mpsc::channel();
        crate::graph::physics::start_physics(state.clone(), config, kill_rx);
        self.graph_state = Some(state);
        self.graph_kill_tx = Some(kill_tx);
    }

    pub fn shutdown(&mut self) {
        if let Some(kill_tx) = self.graph_kill_tx.take() {
            let _ = kill_tx.send(());
        }
        self.graph_state = None;
    }
}
