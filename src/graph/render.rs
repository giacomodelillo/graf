use std::collections::{HashMap, HashSet};

use fdg_sim::petgraph::graph::NodeIndex;
use fdg_sim::petgraph::visit::{EdgeRef, IntoEdgeReferences};
use ratatui::style::Color;
use ratatui::widgets::canvas::{Canvas, Line, Painter, Shape};
use ratatui::widgets::BorderType;

use super::GraphState;
use crate::config::{
    EdgeColorMode, GrafConfig, LabelMode, LegendPosition, NodeColorMode, NodeSizeMode,
};

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    let h = h / 360.0;
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    let (r, g, b) = match (h * 6.0) as u8 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    (
        ((r + m) * 255.0).round() as u8,
        ((g + m) * 255.0).round() as u8,
        ((b + m) * 255.0).round() as u8,
    )
}

fn golden_ratio_hash(s: &str) -> f64 {
    let golden = 0.618033988749895;
    let hash: u32 = s
        .bytes()
        .fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32));
    (hash as f64 * golden) % 1.0
}

fn tag_color(tag: &str, index: usize, total: usize) -> Color {
    let hue_spread = 360.0 / total as f64;
    let base_hue = (index as f64) * hue_spread;
    let perturbation = golden_ratio_hash(tag) * hue_spread * 0.5 - hue_spread * 0.25;
    let hue = (base_hue + perturbation + 360.0) % 360.0;
    let (r, g, b) = hsl_to_rgb(hue, 0.75, 0.55);
    Color::Rgb(r, g, b)
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
            .map(|(i, tag)| (tag.clone(), tag_color(&tag, i, total)))
            .collect()
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

    let block = ratatui::widgets::Block::default()
        .borders(ratatui::widgets::Borders::ALL)
        .border_type(border_type)
        .border_style(ratatui::style::Style::default().fg(colors.border_color))
        .title(config.expand_border_title())
        .title_style(ratatui::style::Style::default().fg(colors.title_color));

    let canvas = Canvas::default()
        .x_bounds(x_bounds)
        .y_bounds(y_bounds)
        .block(block)
        .marker(ratatui::symbols::Marker::Braille)
        .paint(move |ctx| {
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

            if config.visual.show_legend && !tag_colors.is_empty() {
                draw_legend(
                    ctx,
                    &tag_colors,
                    config.legend.max_items,
                    config.legend.position.clone(),
                    x_bounds,
                    y_bounds,
                );
            }
        });

    frame.render_widget(canvas, area);

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

fn draw_legend(
    ctx: &mut ratatui::widgets::canvas::Context<'_>,
    tag_colors: &HashMap<String, Color>,
    max_items: usize,
    position: LegendPosition,
    x_bounds: [f64; 2],
    y_bounds: [f64; 2],
) {
    let mut sorted: Vec<_> = tag_colors.iter().collect();
    sorted.sort_by_key(|(t, _)| t.clone());
    sorted.truncate(max_items);

    if sorted.is_empty() {
        return;
    }

    let width = sorted.iter().map(|(t, _)| t.len()).max().unwrap_or(0) + 3;
    let height = sorted.len() as f64 * 1.5 + 2.0;

    let (start_x, start_y) = match position {
        LegendPosition::TopLeft => (x_bounds[0] + 2.0, y_bounds[1] - 2.0),
        LegendPosition::TopRight => (x_bounds[1] - width as f64 * 2.0 - 2.0, y_bounds[1] - 2.0),
        LegendPosition::BottomLeft => (x_bounds[0] + 2.0, y_bounds[0] + height + 2.0),
        LegendPosition::BottomRight => (
            x_bounds[1] - width as f64 * 2.0 - 2.0,
            y_bounds[0] + height + 2.0,
        ),
    };

    for (i, (tag, color)) in sorted.iter().enumerate() {
        let dot_y = start_y - i as f64 * 1.5;
        ctx.draw(&Line {
            x1: start_x,
            y1: dot_y,
            x2: start_x + 1.0,
            y2: dot_y,
            color: **color,
        });
        let text = tag.as_str().to_string();
        let span = ratatui::text::Span::raw(text);
        ctx.print(start_x + 1.5, dot_y - 0.5, span);
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
