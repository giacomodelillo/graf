use std::sync::{Arc, RwLock};
use std::time::Instant;

use crossterm::event::{KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

use super::GraphState;
use crate::config::GrafConfig;

pub fn handle_graph_keys(
    state: &Arc<RwLock<GraphState>>,
    key: KeyEvent,
    config: &GrafConfig,
) -> Option<String> {
    let mut guard = state.write().unwrap_or_else(|e| e.into_inner());

    match key.code {
        crossterm::event::KeyCode::Esc | crossterm::event::KeyCode::Char('q') => {
            return Some("quit".to_string());
        }
        crossterm::event::KeyCode::Up => {
            guard.viewport.pan_up(config.interaction.pan_sensitivity);
        }
        crossterm::event::KeyCode::Down => {
            guard.viewport.pan_down(config.interaction.pan_sensitivity);
        }
        crossterm::event::KeyCode::Left => {
            guard.viewport.pan_left(config.interaction.pan_sensitivity);
        }
        crossterm::event::KeyCode::Right => {
            guard.viewport.pan_right(config.interaction.pan_sensitivity);
        }
        crossterm::event::KeyCode::Char('+') | crossterm::event::KeyCode::Char('=') => {
            guard.viewport.zoom_in(config.interaction.zoom_factor);
        }
        crossterm::event::KeyCode::Char('-') => {
            guard.viewport.zoom_out(config.interaction.zoom_factor);
        }
        crossterm::event::KeyCode::Enter => {
            if let Some(idx) = guard.selected_node {
                if let Some(node) = guard.simulation.get_graph().node_weight(idx) {
                    return Some(format!("open:{}", node.data.relative_path));
                }
            }
        }
        crossterm::event::KeyCode::Char('a') => {
            let vp = guard
                .viewport
                .clone()
                .auto_fit_from_graph(guard.simulation.get_graph());
            guard.viewport = vp;
        }
        crossterm::event::KeyCode::Char('/') => {
            return Some("search".to_string());
        }
        crossterm::event::KeyCode::Char('f')
            if key
                .modifiers
                .contains(crossterm::event::KeyModifiers::CONTROL) =>
        {
            return Some("search".to_string());
        }
        crossterm::event::KeyCode::Char('?') => {
            return Some("help".to_string());
        }
        _ => {}
    }

    None
}

#[derive(Default)]
pub struct GraphMouseState {
    pub drag_origin: Option<(u16, u16)>,
    pub is_panning: bool,
    pub last_click_time: Option<Instant>,
    pub last_clicked_node: Option<fdg_sim::petgraph::graph::NodeIndex>,
    pub is_minimap_dragging: bool,
}

pub fn handle_graph_mouse(
    state: &Arc<RwLock<GraphState>>,
    mouse_event: MouseEvent,
    area: Rect,
    mouse_state: &mut GraphMouseState,
    config: &GrafConfig,
) -> Option<String> {
    let minimap_area = if config.visual.show_minimap {
        Some(super::render::compute_minimap_area(area, config))
    } else {
        None
    };

    let in_minimap = minimap_area.map_or(false, |ma| {
        mouse_event.column >= ma.x
            && mouse_event.column < ma.x + ma.width
            && mouse_event.row >= ma.y
            && mouse_event.row < ma.y + ma.height
    });

    match mouse_event.kind {
        MouseEventKind::ScrollUp => {
            let mut guard = state.write().unwrap_or_else(|e| e.into_inner());
            guard.viewport.zoom_in(config.interaction.zoom_factor);
        }
        MouseEventKind::ScrollDown => {
            let mut guard = state.write().unwrap_or_else(|e| e.into_inner());
            guard.viewport.zoom_out(config.interaction.zoom_factor);
        }
        MouseEventKind::Down(MouseButton::Left) => {
            if in_minimap {
                if let Some(ma) = minimap_area {
                    let world = minimap_screen_to_world(
                        mouse_event.column,
                        mouse_event.row,
                        ma,
                        &state.read().unwrap_or_else(|e| e.into_inner()),
                    );
                    let mut guard = state.write().unwrap_or_else(|e| e.into_inner());
                    guard.viewport.center_x = world.0;
                    guard.viewport.center_y = world.1;
                    mouse_state.is_minimap_dragging = true;
                    mouse_state.drag_origin = Some((mouse_event.column, mouse_event.row));
                }
            } else {
                let (wx, wy) = {
                    let guard = state.read().unwrap_or_else(|e| e.into_inner());
                    guard
                        .viewport
                        .screen_to_world(mouse_event.column, mouse_event.row, area)
                };

                let hit = {
                    let guard = state.read().unwrap_or_else(|e| e.into_inner());
                    guard.viewport.hit_test(wx, wy, &guard)
                };

                let is_double_click = mouse_state.last_click_time.is_some_and(|t| {
                    t.elapsed().as_millis() < config.interaction.double_click_ms as u128
                });

                if let Some(node_idx) = hit {
                    let mut guard = state.write().unwrap_or_else(|e| e.into_inner());
                    guard.selected_node = Some(node_idx);
                    guard.dragging_node = Some(node_idx);
                    mouse_state.drag_origin = Some((mouse_event.column, mouse_event.row));
                    mouse_state.is_panning = false;
                    mouse_state.last_clicked_node = Some(node_idx);

                    if is_double_click {
                        if let Some(node) = guard.simulation.get_graph().node_weight(node_idx) {
                            mouse_state.last_click_time = Some(Instant::now());
                            return Some(format!("open:{}", node.data.relative_path));
                        }
                    }
                } else {
                    let mut guard = state.write().unwrap_or_else(|e| e.into_inner());
                    if is_double_click {
                        guard.selected_node = None;
                    }
                    guard.dragging_node = None;
                    mouse_state.drag_origin = Some((mouse_event.column, mouse_event.row));
                    mouse_state.is_panning = true;
                    mouse_state.last_clicked_node = None;
                }
            }
        }
        MouseEventKind::Drag(MouseButton::Left) => {
            let Some((orig_col, orig_row)) = mouse_state.drag_origin else {
                return None;
            };

            if mouse_state.is_minimap_dragging {
                if let Some(ma) = minimap_area {
                    let world = minimap_screen_to_world(
                        mouse_event.column,
                        mouse_event.row,
                        ma,
                        &state.read().unwrap_or_else(|e| e.into_inner()),
                    );
                    let mut guard = state.write().unwrap_or_else(|e| e.into_inner());
                    guard.viewport.center_x = world.0;
                    guard.viewport.center_y = world.1;
                    mouse_state.drag_origin = Some((mouse_event.column, mouse_event.row));
                }
            } else if mouse_state.is_panning {
                let dx = -(mouse_event.column as f64 - orig_col as f64)
                    * config.interaction.pan_sensitivity;
                let dy =
                    (mouse_event.row as f64 - orig_row as f64) * config.interaction.pan_sensitivity;
                let mut guard = state.write().unwrap_or_else(|e| e.into_inner());
                guard.viewport.pan(dx, dy);
                mouse_state.drag_origin = Some((mouse_event.column, mouse_event.row));
            } else {
                let (wx, wy) = {
                    let guard = state.read().unwrap_or_else(|e| e.into_inner());
                    guard
                        .viewport
                        .screen_to_world(mouse_event.column, mouse_event.row, area)
                };

                let mut guard = state.write().unwrap_or_else(|e| e.into_inner());
                if let Some(node_idx) = guard.dragging_node {
                    let graph = guard.simulation.get_graph_mut();
                    if let Some(node) = graph.node_weight_mut(node_idx) {
                        node.location.x = wx as f32;
                        node.location.y = wy as f32;
                        node.velocity = fdg_sim::glam::Vec3::ZERO;
                    }
                    guard.drag_target = Some((wx as f32, wy as f32));
                    guard.is_settled = false;
                }
                mouse_state.drag_origin = Some((mouse_event.column, mouse_event.row));
            }
        }
        MouseEventKind::Up(MouseButton::Left) => {
            {
                let mut guard = state.write().unwrap_or_else(|e| e.into_inner());
                guard.dragging_node = None;
                guard.drag_target = None;
            }
            mouse_state.drag_origin = None;
            mouse_state.is_panning = false;
            mouse_state.is_minimap_dragging = false;
            mouse_state.last_click_time = Some(Instant::now());
        }
        _ => {}
    }

    None
}

fn minimap_screen_to_world(
    col: u16,
    row: u16,
    minimap_area: Rect,
    state: &GraphState,
) -> (f64, f64) {
    let (wx_min, wx_max, wy_min, wy_max) =
        super::render::compute_graph_bounds(state.simulation.get_graph());
    let inner_x = minimap_area.x + 1;
    let inner_y = minimap_area.y + 1;
    let inner_w = minimap_area.width.saturating_sub(2);
    let inner_h = minimap_area.height.saturating_sub(2);

    if inner_w == 0 || inner_h == 0 {
        return (0.0, 0.0);
    }

    let rel_x = (col as f64 - inner_x as f64) / inner_w as f64;
    let rel_y = 1.0 - (row as f64 - inner_y as f64) / inner_h as f64;

    let wx = wx_min + rel_x * (wx_max - wx_min);
    let wy = wy_min + rel_y * (wy_max - wy_min);
    (wx, wy)
}
