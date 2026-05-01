use std::collections::{HashMap, HashSet};

use fdg_sim::petgraph::graph::NodeIndex;
use fdg_sim::petgraph::visit::{EdgeRef, IntoEdgeReferences};
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::widgets::canvas::{Canvas, Line, Painter, Rectangle, Shape};
use ratatui::widgets::BorderType;

use crate::config::{
    EdgeColorMode, GrafConfig, LabelMode, LegendPosition, NodeColorMode, NodeSizeMode,
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
}

struct GraphNodesShape {
    nodes: Vec<NodeRenderData>,
}

impl Shape for GraphNodesShape {
    fn draw(&self, painter: &mut Painter) {
        for node in &self.nodes {
            let radius = node.radius;
            let steps = 16u32;
            for i in 0..steps {
                let a1 = (i as f64) * std::f64::consts::TAU / (steps as f64);
                let a2 = ((i + 1) as f64) * std::f64::consts::TAU / (steps as f64);
                Line {
                    x1: node.x + radius * a1.cos(),
                    y1: node.y + radius * a1.sin(),
                    x2: node.x + radius * a2.cos(),
                    y2: node.y + radius * a2.sin(),
                    color: node.color,
                }
                .draw(painter);
            }

            let indicator_radius = 1.2;
            let orbit_radius = radius + 2.5;
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
                let ring_radius = radius + 1.5;
                for i in 0..steps {
                    let a1 = (i as f64) * std::f64::consts::TAU / (steps as f64);
                    let a2 = ((i + 1) as f64) * std::f64::consts::TAU / (steps as f64);
                    Line {
                        x1: node.x + ring_radius * a1.cos(),
                        y1: node.y + ring_radius * a1.sin(),
                        x2: node.x + ring_radius * a2.cos(),
                        y2: node.y + ring_radius * a2.sin(),
                        color: Color::White,
                    }
                    .draw(painter);
                }
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

pub fn draw_graph_view(frame: &mut ratatui::Frame, state: &GraphState, config: &GrafConfig) {
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
    let legend_data: Option<Vec<(String, Color)>> = if config.visual.show_legend {
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
                    y: node.location.y as f64 + radius_for_node(&nodes, idx) + 4.0,
                    text: truncate_owned(&node.data.title, config.visual.label_max_length),
                }
            })
            .collect()
    };

    let node_count = graph.node_count();
    let edge_count = graph.edge_count();

    let x_bounds = viewport.x_bounds(aspect);
    let y_bounds = viewport.y_bounds(aspect);

    let border_type = match config.display.border_style {
        crate::config::BorderStyle::Plain => BorderType::Plain,
        crate::config::BorderStyle::Rounded => BorderType::Rounded,
        crate::config::BorderStyle::Double => BorderType::Double,
        crate::config::BorderStyle::None => BorderType::Plain,
    };

    let block = {
        let b = ratatui::widgets::Block::default();
        if matches!(
            config.display.border_style,
            crate::config::BorderStyle::None
        ) {
            b
        } else {
            b.borders(ratatui::widgets::Borders::ALL)
                .border_type(border_type)
                .border_style(ratatui::style::Style::default().fg(colors.border_color))
                .title(config.expand_border_title())
                .title_style(ratatui::style::Style::default().fg(colors.title_color))
        }
    };

    let canvas = Canvas::default()
        .x_bounds(x_bounds)
        .y_bounds(y_bounds)
        .block(block)
        .marker(ratatui::symbols::Marker::Braille)
        .paint(move |ctx| {
            if let Some(bg) = colors.background_color {
                let width = x_bounds[1] - x_bounds[0];
                let height = y_bounds[1] - y_bounds[0];
                ctx.draw(&Rectangle {
                    x: x_bounds[0],
                    y: y_bounds[0],
                    width,
                    height,
                    color: bg,
                });
            }
            if config.visual.show_grid {
                draw_grid(ctx, x_bounds, y_bounds, colors.grid_color);
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

    if config.display.show_status_bar {
        let selected_info = state
            .selected_node
            .and_then(|idx| graph.node_weight(idx))
            .map(|n| n.data.title.clone());
        let status = config.expand_status(node_count, edge_count, selected_info.as_deref());

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
}

fn radius_for_node(nodes: &[NodeRenderData], idx: NodeIndex) -> f64 {
    nodes.get(idx.index()).map(|n| n.radius).unwrap_or(2.0)
}

fn draw_grid(ctx: &mut ratatui::widgets::canvas::Context, x: [f64; 2], y: [f64; 2], color: Color) {
    let step_x = (x[1] - x[0]) / 10.0;
    let step_y = (y[1] - y[0]) / 10.0;
    for i in 0..=10 {
        let px = x[0] + step_x * i as f64;
        ctx.draw(&Line {
            x1: px,
            y1: y[0],
            x2: px,
            y2: y[1],
            color,
        });
    }
    for i in 0..=10 {
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

fn truncate_owned(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        let mut end = max_len.saturating_sub(1);
        while !s.is_char_boundary(end) {
            end -= 1;
        }
        format!("{}…", &s[..end])
    }
}
