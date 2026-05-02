use std::collections::{HashMap, HashSet};

use fdg_sim::petgraph::graph::NodeIndex;
use fdg_sim::petgraph::visit::{EdgeRef, IntoEdgeReferences};
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::widgets::canvas::{Canvas, Line, Painter, Rectangle, Shape};

use crate::config::{
    EdgeColorMode, GrafConfig, LabelMode, LegendPosition, NodeColorMode, NodeShape, NodeSizeMode,
};
use crate::graph::viewport::Viewport;
use crate::graph::GraphState;
fn tag_color(tag: &str, index: usize, _total: usize, palette: &[Color]) -> Color {
    let palette_len = palette.len();
    if palette_len == 0 {
        return Color::Gray;
    }
    let hash = tag
        .bytes()
        .fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32));
    palette[((hash as usize) + index * 7) % palette_len]
}

fn link_count_color(count: usize, max_count: usize, colors: &[Color]) -> Color {
    if max_count == 0 {
        return colors.first().copied().unwrap_or(Color::Gray);
    }
    let idx = (count as f64 / max_count as f64 * (colors.len() - 1) as f64) as usize;
    colors.get(idx).copied().unwrap_or(Color::Gray)
}

#[derive(Clone)]
struct EdgeData {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    color: Color,
    thickness: u16,
}

struct GraphEdgesShape {
    edges: Vec<EdgeData>,
}

impl Shape for GraphEdgesShape {
    fn draw(&self, painter: &mut Painter) {
        for edge in &self.edges {
            for _ in 0..edge.thickness {
                Line {
                    x1: edge.x1,
                    y1: edge.y1,
                    x2: edge.x2,
                    y2: edge.y2,
                    color: edge.color,
                }
                .draw(painter);
            }
        }
    }
}

#[derive(Clone)]
struct NodeRenderData {
    x: f64,
    y: f64,
    color: Color,
    radius: f64,
    extra_tag_colors: Vec<Color>,
    is_selected: bool,
    selection_ring_color: Color,
    shape: NodeShape,
}

struct GraphNodesShape {
    nodes: Vec<NodeRenderData>,
}

fn draw_outlined_shape(
    painter: &mut Painter,
    cx: f64,
    cy: f64,
    radius: f64,
    shape: NodeShape,
    color: Color,
) {
    match shape {
        NodeShape::Circle => {
            let steps = 16u32;
            for i in 0..steps {
                let a1 = (i as f64) * std::f64::consts::TAU / (steps as f64);
                let a2 = ((i + 1) as f64) * std::f64::consts::TAU / (steps as f64);
                Line {
                    x1: cx + radius * a1.cos(),
                    y1: cy + radius * a1.sin(),
                    x2: cx + radius * a2.cos(),
                    y2: cy + radius * a2.sin(),
                    color,
                }
                .draw(painter);
            }
        }
        NodeShape::Square => {
            Line {
                x1: cx - radius,
                y1: cy - radius,
                x2: cx + radius,
                y2: cy - radius,
                color,
            }
            .draw(painter);
            Line {
                x1: cx + radius,
                y1: cy - radius,
                x2: cx + radius,
                y2: cy + radius,
                color,
            }
            .draw(painter);
            Line {
                x1: cx + radius,
                y1: cy + radius,
                x2: cx - radius,
                y2: cy + radius,
                color,
            }
            .draw(painter);
            Line {
                x1: cx - radius,
                y1: cy + radius,
                x2: cx - radius,
                y2: cy - radius,
                color,
            }
            .draw(painter);
        }
        NodeShape::Diamond => {
            Line {
                x1: cx,
                y1: cy - radius,
                x2: cx + radius,
                y2: cy,
                color,
            }
            .draw(painter);
            Line {
                x1: cx + radius,
                y1: cy,
                x2: cx,
                y2: cy + radius,
                color,
            }
            .draw(painter);
            Line {
                x1: cx,
                y1: cy + radius,
                x2: cx - radius,
                y2: cy,
                color,
            }
            .draw(painter);
            Line {
                x1: cx - radius,
                y1: cy,
                x2: cx,
                y2: cy - radius,
                color,
            }
            .draw(painter);
        }
    }
}

impl Shape for GraphNodesShape {
    fn draw(&self, painter: &mut Painter) {
        for node in &self.nodes {
            draw_outlined_shape(painter, node.x, node.y, node.radius, node.shape, node.color);

            let indicator_radius = 1.2;
            let orbit_radius = node.radius + 2.5;
            let extra_count = node.extra_tag_colors.len();
            for (i, &color) in node.extra_tag_colors.iter().enumerate() {
                let angle = (i as f64) * std::f64::consts::TAU / (extra_count as f64)
                    - std::f64::consts::FRAC_PI_2;
                let cx = node.x + orbit_radius * angle.cos();
                let cy = node.y + orbit_radius * angle.sin();
                let dot_steps = 8u32;
                for j in 0..dot_steps {
                    let a1 = (j as f64) * std::f64::consts::TAU / (dot_steps as f64);
                    let a2 = ((j + 1) as f64) * std::f64::consts::TAU / (dot_steps as f64);
                    Line {
                        x1: cx + indicator_radius * a1.cos(),
                        y1: cy + indicator_radius * a1.sin(),
                        x2: cx + indicator_radius * a2.cos(),
                        y2: cy + indicator_radius * a2.sin(),
                        color,
                    }
                    .draw(painter);
                }
            }

            if node.is_selected {
                let ring_radius = node.radius + 1.5;
                draw_outlined_shape(
                    painter,
                    node.x,
                    node.y,
                    ring_radius,
                    node.shape,
                    node.selection_ring_color,
                );
            }
        }
    }
}

#[derive(Clone)]
struct LabelData {
    x: f64,
    y: f64,
    text: String,
}

pub struct FeatureFlags {
    pub show_legend: bool,
    pub show_grid: bool,
    pub show_minimap: bool,
    pub show_status_bar: bool,
}

pub fn draw_graph_view(
    frame: &mut ratatui::Frame,
    state: &GraphState,
    config: &GrafConfig,
    flags: &FeatureFlags,
) {
    let area = frame.area();
    let aspect = area.width as f64 / area.height as f64;
    let viewport = &state.viewport;
    let colors = config.theme_colors();
    let graph = state.simulation.get_graph();

    let max_link_count = graph
        .node_weights()
        .map(|n| n.data.link_count)
        .max()
        .unwrap_or(0);

    let tag_colors: HashMap<String, Color> = {
        let mut unique_tags: HashSet<String> = HashSet::new();
        for node in graph.node_weights() {
            for tag in &node.data.tags {
                unique_tags.insert(tag.clone());
            }
        }
        let mut unique_tags: Vec<String> = unique_tags.into_iter().collect();
        unique_tags.sort();
        let total = unique_tags.len().max(1);
        unique_tags
            .into_iter()
            .enumerate()
            .map(|(i, tag)| (tag.clone(), tag_color(&tag, i, total, &colors.node_colors)))
            .collect()
    };

    let folder_colors: HashMap<String, Color> = {
        let mut unique_folders: HashSet<String> = HashSet::new();
        for node in graph.node_weights() {
            unique_folders.insert(node.data.folder.clone());
        }
        let mut unique_folders: Vec<String> = unique_folders.into_iter().collect();
        unique_folders.sort();
        let total = unique_folders.len().max(1);
        unique_folders
            .into_iter()
            .enumerate()
            .map(|(i, f)| (f.clone(), tag_color(&f, i, total, &colors.node_colors)))
            .collect()
    };

    // Prepare legend data if needed
    let legend_data: Option<Vec<(String, Color)>> = if flags.show_legend {
        let items = match config.visual.node_color_mode {
            NodeColorMode::Folder => &folder_colors,
            _ => &tag_colors,
        };
        if items.is_empty() {
            None
        } else {
            let mut sorted: Vec<_> = items.iter().collect();
            sorted.sort_by_key(|(t, _)| t.as_str());
            sorted.truncate(config.legend.max_items);
            Some(sorted.into_iter().map(|(t, c)| (t.clone(), *c)).collect())
        }
    } else {
        None
    };

    let mut node_own_color: HashMap<NodeIndex, Color> = HashMap::new();
    for idx in graph.node_indices() {
        let node = &graph[idx];
        node_own_color.insert(
            idx,
            match config.visual.node_color_mode {
                NodeColorMode::Tag => {
                    if let Some(tag) = node.data.tags.first() {
                        tag_colors.get(tag).copied().unwrap_or(Color::Gray)
                    } else {
                        Color::Gray
                    }
                }
                NodeColorMode::Folder => folder_colors
                    .get(&node.data.folder)
                    .copied()
                    .unwrap_or(Color::Gray),
                NodeColorMode::LinkCount => {
                    link_count_color(node.data.link_count, max_link_count, &colors.node_colors)
                }
                NodeColorMode::Uniform => {
                    colors.node_colors.first().copied().unwrap_or(Color::Gray)
                }
            },
        );
    }

    let edges: Vec<EdgeData> = graph
        .edge_references()
        .map(|edge| {
            let src = &graph[edge.source()];
            let tgt = &graph[edge.target()];
            let color = match config.visual.edge_color_mode {
                EdgeColorMode::Source => *node_own_color
                    .get(&edge.source())
                    .unwrap_or(&colors.edge_color),
                EdgeColorMode::Target => *node_own_color
                    .get(&edge.target())
                    .unwrap_or(&colors.edge_color),
                EdgeColorMode::Uniform => colors.edge_color,
            };
            EdgeData {
                x1: src.location.x as f64,
                y1: src.location.y as f64,
                x2: tgt.location.x as f64,
                y2: tgt.location.y as f64,
                color,
                thickness: config.visual.edge_thickness,
            }
        })
        .collect();

    let nodes: Vec<NodeRenderData> = graph
        .node_indices()
        .map(|idx| {
            let node = &graph[idx];
            let primary_color = node_own_color.get(&idx).copied().unwrap_or(Color::Gray);
            let radius = match config.visual.node_size_mode {
                NodeSizeMode::Fixed => config.visual.node_size,
                NodeSizeMode::LinkCount => {
                    if max_link_count == 0 {
                        config.visual.node_size
                    } else {
                        config.visual.node_size
                            * (1.0 + (node.data.link_count as f64 / max_link_count as f64) * 1.5)
                    }
                }
            };
            let extra_tag_colors: Vec<Color> = if node.data.tags.is_empty() {
                Vec::new()
            } else {
                node.data
                    .tags
                    .iter()
                    .skip(1)
                    .filter_map(|tag| tag_colors.get(tag).copied())
                    .collect()
            };
            NodeRenderData {
                x: node.location.x as f64,
                y: node.location.y as f64,
                color: primary_color,
                radius,
                extra_tag_colors,
                is_selected: state.selected_node == Some(idx),
                selection_ring_color: colors.selected_indicator_color,
                shape: config.visual.node_shape,
            }
        })
        .collect();

    let labels: Vec<LabelData> = {
        let should_show = |idx: NodeIndex| -> bool {
            match config.visual.label_mode {
                LabelMode::Selected => state.selected_node == Some(idx),
                LabelMode::Neighbors => {
                    if state.selected_node == Some(idx) {
                        return true;
                    }
                    if let Some(sel) = state.selected_node {
                        for edge in graph.edges(sel) {
                            if edge.target() == idx || edge.source() == idx {
                                return true;
                            }
                        }
                    }
                    false
                }
                LabelMode::All => true,
                LabelMode::None => false,
            }
        };

        graph
            .node_indices()
            .filter(|idx| should_show(*idx))
            .map(|idx| {
                let node = &graph[idx];
                LabelData {
                    x: node.location.x as f64,
                    y: node.location.y as f64
                        + radius_for_node(&nodes, idx)
                        + config.visual.label_offset,
                    text: crate::util::truncate(&node.data.title, config.visual.label_max_length),
                }
            })
            .collect()
    };

    let node_count = graph.node_count();
    let edge_count = graph.edge_count();

    let x_bounds = viewport.x_bounds(aspect);
    let y_bounds = viewport.y_bounds(aspect);

    let border_type = config.display.border_style.to_border_type();

    let block = {
        let b = ratatui::widgets::Block::default();
        if matches!(
            config.display.border_style,
            crate::config::BorderStyle::None
        ) {
            b
        } else {
            let mut block = b
                .borders(ratatui::widgets::Borders::ALL)
                .border_type(border_type)
                .border_style(ratatui::style::Style::default().fg(colors.border_color))
                .title(config.expand_border_title())
                .title_style(ratatui::style::Style::default().fg(colors.title_color));

            // Add background color to block for solid background mode
            if let Some(bg) = colors.background_color {
                block = block.style(ratatui::style::Style::default().bg(bg));
            }

            block
        }
    };

    let canvas = Canvas::default()
        .background_color(colors.background_color.unwrap_or(Color::Reset))
        .x_bounds(x_bounds)
        .y_bounds(y_bounds)
        .block(block)
        .marker(ratatui::symbols::Marker::from(
            config.visual.canvas_marker.clone(),
        ))
        .paint(move |ctx| {
            if flags.show_grid {
                draw_grid(
                    ctx,
                    x_bounds,
                    y_bounds,
                    colors.grid_color,
                    config.visual.grid_divisions,
                );
            }
            ctx.draw(&GraphEdgesShape {
                edges: edges.clone(),
            });
            ctx.layer();
            ctx.draw(&GraphNodesShape {
                nodes: nodes.clone(),
            });
            ctx.layer();
            for label in &labels {
                let span = ratatui::text::Span::styled(
                    label.text.clone(),
                    ratatui::style::Style::default().fg(colors.label_color),
                );
                ctx.print(label.x, label.y, span);
            }
        });

    frame.render_widget(canvas, area);

    // Draw legend overlay if data exists
    if let Some(ref items) = legend_data {
        let max_len = items.iter().map(|(t, _)| t.len()).max().unwrap_or(0);
        let legend_width = (max_len + 4) as u16;
        let legend_height = (items.len() as u16).min(config.legend.max_items as u16) + 2;
        let (legend_x, legend_y) = match config.legend.position {
            LegendPosition::TopLeft => (area.x + 1, area.y + 1),
            LegendPosition::TopRight => (
                area.x + area.width.saturating_sub(legend_width + 1),
                area.y + 1,
            ),
            LegendPosition::BottomLeft => (
                area.x + 1,
                area.y + area.height.saturating_sub(legend_height + 1),
            ),
            LegendPosition::BottomRight => (
                area.x + area.width.saturating_sub(legend_width + 1),
                area.y + area.height.saturating_sub(legend_height + 1),
            ),
        };
        let legend_area =
            ratatui::layout::Rect::new(legend_x, legend_y, legend_width, legend_height);
        let legend_text: Vec<ratatui::text::Line> = items
            .iter()
            .map(|(t, c)| {
                ratatui::text::Line::from(vec![
                    ratatui::text::Span::styled("● ", ratatui::style::Style::default().fg(*c)),
                    ratatui::text::Span::styled(
                        t.clone(),
                        ratatui::style::Style::default().fg(colors.label_color),
                    ),
                ])
            })
            .collect();
        let legend_widget = ratatui::widgets::Paragraph::new(legend_text).block(
            ratatui::widgets::Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .border_style(ratatui::style::Style::default().fg(colors.border_color)),
        );
        frame.render_widget(legend_widget, legend_area);
    }

    if flags.show_status_bar {
        let selected_info = state
            .selected_node
            .and_then(|idx| graph.node_weight(idx))
            .map(|n| n.data.title.clone());

        let (viewport_size_pct, viewport_ratio) = {
            let (gx_min, gx_max, gy_min, gy_max) = state.graph_bounds;
            let graph_w = gx_max - gx_min;
            let graph_h = gy_max - gy_min;
            let vp_w = x_bounds[1] - x_bounds[0];
            let vp_h = y_bounds[1] - y_bounds[0];
            let graph_area = graph_w * graph_h;
            let vp_area = vp_w * vp_h;
            let size_pct = if graph_area > 0.0 {
                (vp_area / graph_area * 100.0).clamp(0.0, 100.0)
            } else {
                100.0
            };
            let range = graph_w.max(graph_h).max(1.0) * 1.4;
            let full_zoom = 200.0 / range;
            let ratio = viewport.zoom / full_zoom;
            (size_pct, ratio)
        };

        let status = config.expand_status(
            node_count,
            edge_count,
            selected_info.as_deref(),
            Some(viewport_size_pct),
            Some(viewport_ratio),
        );

        let status_bar = ratatui::widgets::Paragraph::new(status)
            .style(ratatui::style::Style::default().fg(colors.status_bar_color));
        let status_area = ratatui::layout::Rect::new(
            area.x + 1,
            area.y + area.height.saturating_sub(1),
            area.width.saturating_sub(2),
            1,
        );
        frame.render_widget(status_bar, status_area);
    }

    if flags.show_minimap {
        let minimap_area = compute_minimap_area(area, config);
        frame.render_widget(ratatui::widgets::Clear, minimap_area);
        draw_minimap(
            frame,
            minimap_area,
            MinimapParams {
                viewport,
                graph,
                node_colors: &node_own_color,
                colors: &colors,
                config,
                graph_bounds: state.graph_bounds,
            },
        );
    }
}

fn radius_for_node(nodes: &[NodeRenderData], idx: NodeIndex) -> f64 {
    nodes.get(idx.index()).map(|n| n.radius).unwrap_or(2.0)
}

fn draw_grid(
    ctx: &mut ratatui::widgets::canvas::Context,
    x: [f64; 2],
    y: [f64; 2],
    color: Color,
    divisions: usize,
) {
    let divs = divisions.max(2);
    let step_x = (x[1] - x[0]) / divs as f64;
    let step_y = (y[1] - y[0]) / divs as f64;
    for i in 0..=divs {
        let px = x[0] + step_x * i as f64;
        ctx.draw(&Line {
            x1: px,
            y1: y[0],
            x2: px,
            y2: y[1],
            color,
        });
    }
    for i in 0..=divs {
        let py = y[0] + step_y * i as f64;
        ctx.draw(&Line {
            x1: x[0],
            y1: py,
            x2: x[1],
            y2: py,
            color,
        });
    }
}

pub fn compute_minimap_area(frame_area: Rect, config: &GrafConfig) -> Rect {
    let w = config.visual.minimap_width;
    let h = config.visual.minimap_height;
    let (x, y) = match config.visual.minimap_position {
        LegendPosition::TopLeft => (frame_area.x + 1, frame_area.y + 1),
        LegendPosition::TopRight => (
            frame_area.x + frame_area.width.saturating_sub(w + 1),
            frame_area.y + 1,
        ),
        LegendPosition::BottomLeft => (
            frame_area.x + 1,
            frame_area.y + frame_area.height.saturating_sub(h + 1),
        ),
        LegendPosition::BottomRight => (
            frame_area.x + frame_area.width.saturating_sub(w + 1),
            frame_area.y + frame_area.height.saturating_sub(h + 1),
        ),
    };
    Rect::new(x, y, w, h)
}

pub fn compute_graph_bounds(
    graph: &fdg_sim::ForceGraph<super::GraphNodeData, ()>,
) -> (f64, f64, f64, f64) {
    let mut min_x = f64::MAX;
    let mut max_x = f64::MIN;
    let mut min_y = f64::MAX;
    let mut max_y = f64::MIN;

    for node in graph.node_weights() {
        let x = node.location.x as f64;
        let y = node.location.y as f64;
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }

    if min_x == f64::MAX {
        min_x = -100.0;
        max_x = 100.0;
        min_y = -100.0;
        max_y = 100.0;
    }

    let pad_x = (max_x - min_x) * 0.1 + 1.0;
    let pad_y = (max_y - min_y) * 0.1 + 1.0;
    (min_x - pad_x, max_x + pad_x, min_y - pad_y, max_y + pad_y)
}

struct MinimapParams<'a> {
    viewport: &'a Viewport,
    graph: &'a fdg_sim::ForceGraph<super::GraphNodeData, ()>,
    node_colors: &'a HashMap<NodeIndex, Color>,
    colors: &'a crate::config::ThemeColors,
    config: &'a crate::config::GrafConfig,
    graph_bounds: (f64, f64, f64, f64),
}

fn draw_minimap(frame: &mut ratatui::Frame, area: Rect, params: MinimapParams<'_>) {
    let (wx_min, wx_max, wy_min, wy_max) = params.graph_bounds;
    let aspect = area.width as f64 / area.height as f64;
    let vp_x = params.viewport.x_bounds(aspect);
    let vp_y = params.viewport.y_bounds(aspect);

    let nodes_clone: Vec<(f64, f64, Color)> = params
        .graph
        .node_indices()
        .map(|idx| {
            let node = &params.graph[idx];
            let color = params.node_colors.get(&idx).copied().unwrap_or(Color::Gray);
            (node.location.x as f64, node.location.y as f64, color)
        })
        .collect();

    let vp_color = params.colors.minimap_viewport_color;
    let bg_color = params.colors.minimap_bg_color;

    let canvas = Canvas::default()
        .x_bounds([wx_min, wx_max])
        .y_bounds([wy_min, wy_max])
        .block(
            ratatui::widgets::Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .border_style(
                    ratatui::style::Style::default().fg(params.colors.minimap_border_color),
                ),
        )
        .marker(ratatui::symbols::Marker::from(
            params.config.visual.minimap_marker.clone(),
        ))
        .paint(move |ctx| {
            if let Some(bg) = bg_color {
                ctx.draw(&Rectangle {
                    x: wx_min,
                    y: wy_min,
                    width: wx_max - wx_min,
                    height: wy_max - wy_min,
                    color: bg,
                });
            }

            for (nx, ny, nc) in &nodes_clone {
                ctx.draw(&Rectangle {
                    x: *nx - 0.5,
                    y: *ny - 0.5,
                    width: 1.0,
                    height: 1.0,
                    color: *nc,
                });
            }

            let vx1 = vp_x[0].max(wx_min);
            let vx2 = vp_x[1].min(wx_max);
            let vy1 = vp_y[0].max(wy_min);
            let vy2 = vp_y[1].min(wy_max);

            if vx1 < vx2 && vy1 < vy2 {
                ctx.draw(&Line {
                    x1: vx1,
                    y1: vy1,
                    x2: vx2,
                    y2: vy1,
                    color: vp_color,
                });
                ctx.draw(&Line {
                    x1: vx1,
                    y1: vy2,
                    x2: vx2,
                    y2: vy2,
                    color: vp_color,
                });
                ctx.draw(&Line {
                    x1: vx1,
                    y1: vy1,
                    x2: vx1,
                    y2: vy2,
                    color: vp_color,
                });
                ctx.draw(&Line {
                    x1: vx2,
                    y1: vy1,
                    x2: vx2,
                    y2: vy2,
                    color: vp_color,
                });
            }
        });

    frame.render_widget(canvas, area);
}
