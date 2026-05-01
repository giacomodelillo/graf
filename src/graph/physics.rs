use std::sync::{mpsc, Arc, RwLock};

use super::GraphState;
use crate::config::GrafConfig;

pub fn start_physics(
    state: Arc<RwLock<GraphState>>,
    config: &GrafConfig,
    kill_rx: mpsc::Receiver<()>,
) {
    let gravity = config.physics.gravity;

    std::thread::spawn(move || loop {
        if kill_rx.try_recv().is_ok() {
            break;
        }

        let should_update = {
            let guard = state.read().unwrap_or_else(|e| e.into_inner());
            !guard.is_settled
        };

        if should_update {
            let mut guard = state.write().unwrap_or_else(|e| e.into_inner());
            guard.simulation.update(0.016);

            if gravity > 0.0 {
                let graph = guard.simulation.get_graph_mut();
                for node in graph.node_weights_mut() {
                    node.velocity.x -= node.location.x * gravity as f32;
                    node.velocity.y -= node.location.y * gravity as f32;
                }
            }

            let graph = guard.simulation.get_graph();
            let energy: f32 = graph.node_weights().map(|n| n.velocity.length()).sum();

            if energy < 0.05 * graph.node_count() as f32 {
                guard.is_settled = true;
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(16));
    });
}
