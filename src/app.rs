use std::sync::Arc;
use std::sync::RwLock;

use crate::config::GrafConfig;
use crate::graph::input::GraphMouseState;
use crate::linker::FileData;

pub struct AppState {
    pub graph_state: Option<Arc<RwLock<crate::graph::GraphState>>>,
    pub graph_kill_tx: Option<std::sync::mpsc::Sender<()>>,
    pub graph_mouse_state: GraphMouseState,
    pub files: Vec<FileData>,
    pub show_help: bool,
}

impl AppState {
    pub fn new(config: &GrafConfig, files: Vec<FileData>) -> Self {
        let (graph, _total_edges) = crate::graph::build_graph(&files, config);
        let simulation = crate::graph::create_simulation(graph, config);
        let mut graph_state = crate::graph::GraphState {
            viewport: crate::graph::viewport::Viewport::default(),
            simulation,
            selected_node: None,
            dragging_node: None,
            drag_target: None,
            is_settled: false,
        };
        let vp = graph_state
            .viewport
            .clone()
            .auto_fit_from_graph(graph_state.simulation.get_graph());
        graph_state.viewport = vp;

        let state = Arc::new(RwLock::new(graph_state));
        let (kill_tx, kill_rx) = std::sync::mpsc::channel();
        crate::graph::physics::start_physics(state.clone(), config, kill_rx);

        Self {
            graph_state: Some(state),
            graph_kill_tx: Some(kill_tx),
            graph_mouse_state: GraphMouseState::default(),
            files,
            show_help: false,
        }
    }

    pub fn shutdown(&mut self) {
        if let Some(kill_tx) = self.graph_kill_tx.take() {
            let _ = kill_tx.send(());
        }
        self.graph_state = None;
    }
}
