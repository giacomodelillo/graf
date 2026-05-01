use std::fs;
use std::path::PathBuf;

use directories::ProjectDirs;
use ratatui::style::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Theme {
    #[default]
    Default,
    TokyoNight,
    Gruvbox,
    Dracula,
    Nord,
    CatppuccinMocha,
    Onedark,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Background {
    #[default]
    Transparent,
    Solid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum NodeColorMode {
    #[default]
    Tag,
    LinkCount,
    Uniform,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum EdgeColorMode {
    #[default]
    Source,
    Target,
    Uniform,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum LabelMode {
    #[default]
    Selected,
    Neighbors,
    All,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum NodeSizeMode {
    #[default]
    Fixed,
    LinkCount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BorderStyle {
    Plain,
    Rounded,
    Double,
    None,
}

impl Default for BorderStyle {
    fn default() -> Self {
        Self::Rounded
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LegendPosition {
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

impl Default for LegendPosition {
    fn default() -> Self {
        Self::TopRight
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualConfig {
    #[serde(default)]
    pub theme: Theme,
    #[serde(default)]
    pub background: Background,
    #[serde(default)]
    pub node_color_mode: NodeColorMode,
    #[serde(default)]
    pub edge_color_mode: EdgeColorMode,
    #[serde(default)]
    pub label_mode: LabelMode,
    #[serde(default = "default_label_max")]
    pub label_max_length: usize,
    #[serde(default = "default_node_size")]
    pub node_size: f64,
    #[serde(default)]
    pub node_size_mode: NodeSizeMode,
    #[serde(default = "default_edge_thickness")]
    pub edge_thickness: u16,
    #[serde(default = "default_true")]
    pub show_legend: bool,
    #[serde(default)]
    pub show_grid: bool,
}

impl Default for VisualConfig {
    fn default() -> Self {
        Self {
            theme: Theme::Default,
            background: Background::default(),
            node_color_mode: NodeColorMode::default(),
            edge_color_mode: EdgeColorMode::default(),
            label_mode: LabelMode::default(),
            label_max_length: default_label_max(),
            node_size: default_node_size(),
            node_size_mode: NodeSizeMode::default(),
            edge_thickness: default_edge_thickness(),
            show_legend: default_true(),
            show_grid: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsConfig {
    #[serde(default = "default_ideal_distance")]
    pub ideal_distance: f64,
    #[serde(default = "default_repulsion")]
    pub repulsion_strength: f64,
    #[serde(default = "default_attraction")]
    pub attraction_strength: f64,
    #[serde(default = "default_damping")]
    pub damping: f32,
    #[serde(default = "default_max_iterations")]
    pub max_iterations: usize,
    #[serde(default = "default_gravity")]
    pub gravity: f64,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            ideal_distance: default_ideal_distance(),
            repulsion_strength: default_repulsion(),
            attraction_strength: default_attraction(),
            damping: default_damping(),
            max_iterations: default_max_iterations(),
            gravity: default_gravity(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionConfig {
    #[serde(default = "default_double_click")]
    pub double_click_ms: u128,
    #[serde(default = "default_pan_sensitivity")]
    pub pan_sensitivity: f64,
    #[serde(default = "default_zoom_factor")]
    pub zoom_factor: f64,
}

impl Default for InteractionConfig {
    fn default() -> Self {
        Self {
            double_click_ms: default_double_click(),
            pan_sensitivity: default_pan_sensitivity(),
            zoom_factor: default_zoom_factor(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    #[serde(default = "default_true")]
    pub show_status_bar: bool,
    #[serde(default)]
    pub status_format: Option<String>,
    #[serde(default)]
    pub border_style: BorderStyle,
    #[serde(default = "default_border_title")]
    pub border_title: String,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            show_status_bar: default_true(),
            status_format: None,
            border_style: BorderStyle::default(),
            border_title: default_border_title(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterConfig {
    #[serde(default)]
    pub exclude_tags: Vec<String>,
    #[serde(default)]
    pub exclude_patterns: Vec<String>,
    #[serde(default)]
    pub min_links: usize,
    #[serde(default = "default_max_nodes")]
    pub max_nodes: usize,
}

impl Default for FilterConfig {
    fn default() -> Self {
        Self {
            exclude_tags: Vec::new(),
            exclude_patterns: Vec::new(),
            min_links: 0,
            max_nodes: default_max_nodes(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegendConfig {
    #[serde(default)]
    pub position: LegendPosition,
    #[serde(default = "default_max_legend_items")]
    pub max_items: usize,
}

impl Default for LegendConfig {
    fn default() -> Self {
        Self {
            position: LegendPosition::default(),
            max_items: default_max_legend_items(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrafConfig {
    #[serde(default)]
    pub visual: VisualConfig,
    #[serde(default)]
    pub physics: PhysicsConfig,
    #[serde(default)]
    pub interaction: InteractionConfig,
    #[serde(default)]
    pub display: DisplayConfig,
    #[serde(default)]
    pub filter: FilterConfig,
    #[serde(default)]
    pub legend: LegendConfig,
}

impl Default for GrafConfig {
    fn default() -> Self {
        Self {
            visual: VisualConfig::default(),
            physics: PhysicsConfig::default(),
            interaction: InteractionConfig::default(),
            display: DisplayConfig::default(),
            filter: FilterConfig::default(),
            legend: LegendConfig::default(),
        }
    }
}

fn default_label_max() -> usize {
    20
}
fn default_node_size() -> f64 {
    2.0
}
fn default_edge_thickness() -> u16 {
    1
}
fn default_true() -> bool {
    true
}
fn default_ideal_distance() -> f64 {
    80.0
}
fn default_repulsion() -> f64 {
    80.0
}
fn default_attraction() -> f64 {
    1.0
}
fn default_damping() -> f32 {
    0.95
}
fn default_max_iterations() -> usize {
    800
}
fn default_gravity() -> f64 {
    0.01
}
fn default_double_click() -> u128 {
    300
}
fn default_pan_sensitivity() -> f64 {
    0.2
}
fn default_zoom_factor() -> f64 {
    1.15
}
fn default_border_title() -> String {
    "graf".to_string()
}
fn default_max_nodes() -> usize {
    500
}
fn default_max_legend_items() -> usize {
    10
}

pub struct ThemeColors {
    pub node_colors: Vec<Color>,
    pub edge_color: Color,
    pub border_color: Color,
    pub title_color: Color,
    pub label_color: Color,
    pub legend_text_color: Color,
    pub legend_border_color: Color,
    pub selected_indicator_color: Color,
    pub grid_color: Color,
    pub background_color: Option<Color>,
    pub status_bar_color: Color,
}

impl GrafConfig {
    pub fn load() -> Self {
        let config_path = Self::config_path();
        if let Ok(path) = &config_path {
            if path.exists() {
                if let Ok(content) = fs::read_to_string(path) {
                    if let Ok(config) = toml::from_str(&content) {
                        return config;
                    }
                }
            }
        }
        Self::default()
    }

    pub fn config_path() -> anyhow::Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "graf", "graf")
            .ok_or_else(|| anyhow::anyhow!("no home dir"))?;
        Ok(proj_dirs.config_dir().join("config.toml"))
    }

    pub fn theme_colors(&self) -> ThemeColors {
        match self.visual.theme {
            Theme::Default => ThemeColors {
                node_colors: vec![Color::Reset],
                edge_color: Color::DarkGray,
                border_color: Color::DarkGray,
                title_color: Color::Gray,
                label_color: Color::Gray,
                legend_text_color: Color::Gray,
                legend_border_color: Color::DarkGray,
                selected_indicator_color: Color::Reset,
                grid_color: Color::DarkGray,
                background_color: match self.visual.background {
                    Background::Transparent => None,
                    Background::Solid => None,
                },
                status_bar_color: Color::DarkGray,
            },
            Theme::TokyoNight => ThemeColors {
                node_colors: vec![
                    Color::Rgb(122, 162, 247),
                    Color::Rgb(187, 154, 247),
                    Color::Rgb(125, 207, 255),
                    Color::Rgb(224, 175, 104),
                    Color::Rgb(158, 206, 106),
                    Color::Rgb(247, 118, 142),
                    Color::Rgb(148, 226, 213),
                    Color::Rgb(255, 158, 100),
                ],
                edge_color: Color::Rgb(86, 95, 137),
                border_color: Color::Rgb(86, 95, 137),
                title_color: Color::Rgb(187, 154, 247),
                label_color: Color::Rgb(203, 206, 215),
                legend_text_color: Color::Rgb(203, 206, 215),
                legend_border_color: Color::Rgb(86, 95, 137),
                selected_indicator_color: Color::Rgb(255, 255, 255),
                grid_color: Color::Rgb(56, 62, 95),
                background_color: match self.visual.background {
                    Background::Transparent => None,
                    Background::Solid => Some(Color::Rgb(26, 27, 38)),
                },
                status_bar_color: Color::Rgb(86, 95, 137),
            },
            Theme::Gruvbox => ThemeColors {
                node_colors: vec![
                    Color::Rgb(184, 187, 38),
                    Color::Rgb(215, 153, 33),
                    Color::Rgb(204, 94, 74),
                    Color::Rgb(214, 93, 14),
                    Color::Rgb(104, 157, 106),
                    Color::Rgb(131, 165, 152),
                    Color::Rgb(146, 131, 116),
                    Color::Rgb(254, 128, 25),
                ],
                edge_color: Color::Rgb(102, 92, 84),
                border_color: Color::Rgb(102, 92, 84),
                title_color: Color::Rgb(184, 187, 38),
                label_color: Color::Rgb(235, 219, 178),
                legend_text_color: Color::Rgb(235, 219, 178),
                legend_border_color: Color::Rgb(102, 92, 84),
                selected_indicator_color: Color::Rgb(251, 241, 199),
                grid_color: Color::Rgb(60, 56, 54),
                background_color: match self.visual.background {
                    Background::Transparent => None,
                    Background::Solid => Some(Color::Rgb(40, 40, 40)),
                },
                status_bar_color: Color::Rgb(102, 92, 84),
            },
            Theme::Dracula => ThemeColors {
                node_colors: vec![
                    Color::Rgb(139, 233, 253),
                    Color::Rgb(189, 147, 249),
                    Color::Rgb(139, 233, 253),
                    Color::Rgb(255, 184, 108),
                    Color::Rgb(80, 250, 123),
                    Color::Rgb(255, 121, 198),
                    Color::Rgb(255, 139, 127),
                    Color::Rgb(255, 255, 150),
                ],
                edge_color: Color::Rgb(98, 114, 164),
                border_color: Color::Rgb(98, 114, 164),
                title_color: Color::Rgb(189, 147, 249),
                label_color: Color::Rgb(248, 248, 242),
                legend_text_color: Color::Rgb(248, 248, 242),
                legend_border_color: Color::Rgb(98, 114, 164),
                selected_indicator_color: Color::Rgb(255, 255, 255),
                grid_color: Color::Rgb(68, 71, 90),
                background_color: match self.visual.background {
                    Background::Transparent => None,
                    Background::Solid => Some(Color::Rgb(40, 42, 54)),
                },
                status_bar_color: Color::Rgb(98, 114, 164),
            },
            Theme::Nord => ThemeColors {
                node_colors: vec![
                    Color::Rgb(136, 192, 208),
                    Color::Rgb(143, 188, 187),
                    Color::Rgb(163, 190, 140),
                    Color::Rgb(235, 219, 178),
                    Color::Rgb(214, 140, 140),
                    Color::Rgb(216, 170, 133),
                    Color::Rgb(200, 200, 200),
                    Color::Rgb(163, 190, 140),
                ],
                edge_color: Color::Rgb(67, 76, 94),
                border_color: Color::Rgb(67, 76, 94),
                title_color: Color::Rgb(136, 192, 208),
                label_color: Color::Rgb(216, 222, 233),
                legend_text_color: Color::Rgb(216, 222, 233),
                legend_border_color: Color::Rgb(67, 76, 94),
                selected_indicator_color: Color::Rgb(236, 239, 244),
                grid_color: Color::Rgb(59, 66, 82),
                background_color: match self.visual.background {
                    Background::Transparent => None,
                    Background::Solid => Some(Color::Rgb(46, 52, 64)),
                },
                status_bar_color: Color::Rgb(67, 76, 94),
            },
            Theme::CatppuccinMocha => ThemeColors {
                node_colors: vec![
                    Color::Rgb(137, 180, 250),
                    Color::Rgb(203, 166, 247),
                    Color::Rgb(116, 199, 236),
                    Color::Rgb(249, 226, 175),
                    Color::Rgb(166, 227, 161),
                    Color::Rgb(245, 189, 220),
                    Color::Rgb(242, 205, 205),
                    Color::Rgb(250, 179, 135),
                ],
                edge_color: Color::Rgb(108, 112, 134),
                border_color: Color::Rgb(108, 112, 134),
                title_color: Color::Rgb(203, 166, 247),
                label_color: Color::Rgb(205, 214, 244),
                legend_text_color: Color::Rgb(205, 214, 244),
                legend_border_color: Color::Rgb(108, 112, 134),
                selected_indicator_color: Color::Rgb(205, 214, 244),
                grid_color: Color::Rgb(49, 50, 68),
                background_color: match self.visual.background {
                    Background::Transparent => None,
                    Background::Solid => Some(Color::Rgb(30, 30, 46)),
                },
                status_bar_color: Color::Rgb(108, 112, 134),
            },
            Theme::Onedark => ThemeColors {
                node_colors: vec![
                    Color::Rgb(97, 175, 239),
                    Color::Rgb(198, 120, 221),
                    Color::Rgb(86, 182, 194),
                    Color::Rgb(229, 192, 123),
                    Color::Rgb(152, 195, 121),
                    Color::Rgb(224, 108, 117),
                    Color::Rgb(224, 150, 108),
                    Color::Rgb(171, 178, 191),
                ],
                edge_color: Color::Rgb(92, 99, 112),
                border_color: Color::Rgb(92, 99, 112),
                title_color: Color::Rgb(198, 120, 221),
                label_color: Color::Rgb(171, 178, 191),
                legend_text_color: Color::Rgb(171, 178, 191),
                legend_border_color: Color::Rgb(92, 99, 112),
                selected_indicator_color: Color::Rgb(220, 223, 228),
                grid_color: Color::Rgb(56, 63, 76),
                background_color: match self.visual.background {
                    Background::Transparent => None,
                    Background::Solid => Some(Color::Rgb(40, 44, 52)),
                },
                status_bar_color: Color::Rgb(92, 99, 112),
            },
        }
    }

    pub fn expand_border_title(&self) -> String {
        let mut title = self.display.border_title.clone();
        let cwd = std::env::current_dir()
            .ok()
            .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
            .unwrap_or_default();
        title = title.replace("{cwd}", &cwd);
        title
    }

    pub fn expand_status(&self, files: usize, links: usize, selected: Option<&str>) -> String {
        let fmt = self
            .display
            .status_format
            .as_deref()
            .unwrap_or("Files: {files} | Links: {links} | Selected: {selected}");
        let fmt = fmt.replace("{files}", &files.to_string());
        let fmt = fmt.replace("{links}", &links.to_string());
        let fmt = fmt.replace("{selected}", selected.unwrap_or("none"));
        fmt
    }
}
