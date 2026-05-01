use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use directories::ProjectDirs;
use ratatui::style::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Theme {
    #[default]
    Default,
    #[serde(alias = "tokyo-night")]
    TokyoNight,
    #[serde(alias = "catppuccin-mocha")]
    CatppuccinMocha,
    #[serde(alias = "one-dark")]
    Onedark,
    Gruvbox,
    Dracula,
    Nord,
    SolarizedLight,
    SolarizedDark,
}

impl FromStr for Theme {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "default" => Ok(Theme::Default),
            "tokyo-night" | "tokyonight" => Ok(Theme::TokyoNight),
            "catppuccin-mocha" | "catppuccinmocha" => Ok(Theme::CatppuccinMocha),
            "one-dark" | "onedark" => Ok(Theme::Onedark),
            "gruvbox" => Ok(Theme::Gruvbox),
            "dracula" => Ok(Theme::Dracula),
            "nord" => Ok(Theme::Nord),
            "solarized-light" | "solarizedlight" => Ok(Theme::SolarizedLight),
            "solarized-dark" | "solarizeddark" => Ok(Theme::SolarizedDark),
            _ => Err(format!("Unknown theme: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Background {
    #[default]
    Transparent,
    Solid,
}

impl FromStr for Background {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "transparent" => Ok(Background::Transparent),
            "solid" => Ok(Background::Solid),
            _ => Err(format!("Unknown background: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum NodeColorMode {
    #[default]
    Tag,
    Folder,
    LinkCount,
    Uniform,
}

impl FromStr for NodeColorMode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "tag" => Ok(NodeColorMode::Tag),
            "folder" => Ok(NodeColorMode::Folder),
            "linkcount" => Ok(NodeColorMode::LinkCount),
            "uniform" => Ok(NodeColorMode::Uniform),
            _ => Err(format!("Unknown node_color_mode: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum EdgeColorMode {
    #[default]
    Source,
    Target,
    Uniform,
}

impl FromStr for EdgeColorMode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "source" => Ok(EdgeColorMode::Source),
            "target" => Ok(EdgeColorMode::Target),
            "uniform" => Ok(EdgeColorMode::Uniform),
            _ => Err(format!("Unknown edge_color_mode: {}", s)),
        }
    }
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

impl FromStr for LabelMode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "selected" => Ok(LabelMode::Selected),
            "neighbors" => Ok(LabelMode::Neighbors),
            "all" => Ok(LabelMode::All),
            "none" => Ok(LabelMode::None),
            _ => Err(format!("Unknown label_mode: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum NodeSizeMode {
    #[default]
    Fixed,
    LinkCount,
}

impl FromStr for NodeSizeMode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "fixed" => Ok(NodeSizeMode::Fixed),
            "linkcount" => Ok(NodeSizeMode::LinkCount),
            _ => Err(format!("Unknown node_size_mode: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BorderStyle {
    Plain,
    Rounded,
    Double,
    None,
}

impl FromStr for BorderStyle {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "plain" => Ok(BorderStyle::Plain),
            "rounded" => Ok(BorderStyle::Rounded),
            "double" => Ok(BorderStyle::Double),
            "none" => Ok(BorderStyle::None),
            _ => Err(format!("Unknown border_style: {}", s)),
        }
    }
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

impl FromStr for LegendPosition {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "topright" => Ok(LegendPosition::TopRight),
            "topleft" => Ok(LegendPosition::TopLeft),
            "bottomright" => Ok(LegendPosition::BottomRight),
            "bottomleft" => Ok(LegendPosition::BottomLeft),
            _ => Err(format!("Unknown legend position: {}", s)),
        }
    }
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
            damping: default_damping(),
            max_iterations: default_max_iterations(),
            gravity: default_gravity(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionConfig {
    #[serde(default = "default_double_click")]
    pub double_click_ms: u64,
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

fn default_damping() -> f32 {
    0.95
}
fn default_max_iterations() -> usize {
    800
}
fn default_gravity() -> f64 {
    0.01
}
fn default_double_click() -> u64 {
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
    pub fn config_path() -> anyhow::Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "graf", "graf")
            .ok_or_else(|| anyhow::anyhow!("no home dir"))?;
        Ok(proj_dirs.config_dir().join("config.toml"))
    }

    pub fn theme_colors(&self) -> ThemeColors {
        match self.visual.theme {
            Theme::Default => ThemeColors {
                node_colors: vec![
                    Color::Red,
                    Color::Green,
                    Color::Yellow,
                    Color::Blue,
                    Color::Magenta,
                    Color::Cyan,
                    Color::White,
                    Color::Rgb(200, 200, 200),
                ],
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
                    Background::Solid => Some(Color::Reset),
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
            Theme::SolarizedLight => ThemeColors {
                node_colors: vec![
                    Color::Rgb(181, 137, 0),   // yellow
                    Color::Rgb(203, 75, 22),   // orange
                    Color::Rgb(220, 50, 47),   // red
                    Color::Rgb(211, 54, 130),  // magenta
                    Color::Rgb(108, 113, 196), // violet
                    Color::Rgb(38, 139, 210),  // blue
                    Color::Rgb(42, 161, 152),  // cyan
                    Color::Rgb(133, 153, 0),   // green
                ],
                edge_color: Color::Rgb(88, 110, 117),
                border_color: Color::Rgb(88, 110, 117),
                title_color: Color::Rgb(38, 139, 210),
                label_color: Color::Rgb(101, 123, 131),
                legend_text_color: Color::Rgb(101, 123, 131),
                legend_border_color: Color::Rgb(88, 110, 117),
                selected_indicator_color: Color::Rgb(0, 0, 0),
                grid_color: Color::Rgb(238, 232, 213),
                background_color: match self.visual.background {
                    Background::Transparent => None,
                    Background::Solid => Some(Color::Rgb(253, 246, 227)),
                },
                status_bar_color: Color::Rgb(88, 110, 117),
            },
            Theme::SolarizedDark => ThemeColors {
                node_colors: vec![
                    Color::Rgb(181, 137, 0),   // yellow
                    Color::Rgb(203, 75, 22),   // orange
                    Color::Rgb(220, 50, 47),   // red
                    Color::Rgb(211, 54, 130),  // magenta
                    Color::Rgb(108, 113, 196), // violet
                    Color::Rgb(38, 139, 210),  // blue
                    Color::Rgb(42, 161, 152),  // cyan
                    Color::Rgb(133, 153, 0),   // green
                ],
                edge_color: Color::Rgb(147, 161, 161),
                border_color: Color::Rgb(147, 161, 161),
                title_color: Color::Rgb(38, 139, 210),
                label_color: Color::Rgb(131, 148, 150),
                legend_text_color: Color::Rgb(131, 148, 150),
                legend_border_color: Color::Rgb(147, 161, 161),
                selected_indicator_color: Color::Rgb(253, 246, 227),
                grid_color: Color::Rgb(0, 43, 54),
                background_color: match self.visual.background {
                    Background::Transparent => None,
                    Background::Solid => Some(Color::Rgb(0, 43, 54)),
                },
                status_bar_color: Color::Rgb(147, 161, 161),
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

    /// Validate config values, return vec of error msgs.
    pub fn validate(&self) -> Vec<String> {
        let mut errs = Vec::new();
        if self.visual.label_max_length < 1 || self.visual.label_max_length > 60 {
            errs.push(format!(
                "visual.label_max_length must be 1-60, got {}",
                self.visual.label_max_length
            ));
        }
        if self.visual.node_size < 1.0 || self.visual.node_size > 5.0 {
            errs.push(format!(
                "visual.node_size must be 1.0-5.0, got {}",
                self.visual.node_size
            ));
        }
        if self.visual.edge_thickness < 1 || self.visual.edge_thickness > 3 {
            errs.push(format!(
                "visual.edge_thickness must be 1-3, got {}",
                self.visual.edge_thickness
            ));
        }
        if self.interaction.zoom_factor <= 0.0 {
            errs.push(format!(
                "interaction.zoom_factor must be > 0, got {}",
                self.interaction.zoom_factor
            ));
        }
        if self.interaction.pan_sensitivity <= 0.0 {
            errs.push(format!(
                "interaction.pan_sensitivity must be > 0, got {}",
                self.interaction.pan_sensitivity
            ));
        }
        errs
    }

    pub fn load() -> Self {
        let config_path = Self::config_path();
        let mut config = Self::default();
        let mut load_err = None;

        if let Ok(path) = &config_path {
            if path.exists() {
                match fs::read_to_string(path) {
                    Ok(content) => match toml::from_str::<GrafConfig>(&content) {
                        Ok(loaded) => config = loaded,
                        Err(e) => load_err = Some(format!("Invalid config TOML: {}", e)),
                    },
                    Err(e) => load_err = Some(format!("Cannot read config file: {}", e)),
                }
            }
        }

        // Apply env overrides after file load
        config.apply_env_overrides();

        if let Some(err) = load_err {
            eprintln!("Config warning: {}", err);
        }

        config
    }

    fn apply_env_overrides(&mut self) {
        use std::env;
        macro_rules! apply_enum {
            ($var:expr, $field:expr) => {
                if let Ok(s) = env::var(format!("GRAF_{}", $var)) {
                    if let Ok(v) = s.parse() {
                        $field = v;
                    }
                }
            };
        }
        macro_rules! apply_val {
            ($var:expr, $field:expr, $ty:ty) => {
                if let Ok(s) = env::var(format!("GRAF_{}", $var)) {
                    if let Ok(v) = s.parse::<$ty>() {
                        $field = v;
                    }
                }
            };
        }
        apply_enum!("VISUAL_THEME", self.visual.theme);
        apply_enum!("VISUAL_BACKGROUND", self.visual.background);
        apply_enum!("VISUAL_NODE_COLOR_MODE", self.visual.node_color_mode);
        apply_enum!("VISUAL_EDGE_COLOR_MODE", self.visual.edge_color_mode);
        apply_enum!("VISUAL_LABEL_MODE", self.visual.label_mode);
        apply_val!(
            "VISUAL_LABEL_MAX_LENGTH",
            self.visual.label_max_length,
            usize
        );
        apply_val!("VISUAL_NODE_SIZE", self.visual.node_size, f64);
        apply_enum!("VISUAL_NODE_SIZE_MODE", self.visual.node_size_mode);
        apply_val!("VISUAL_EDGE_THICKNESS", self.visual.edge_thickness, u16);
        apply_val!("VISUAL_SHOW_LEGEND", self.visual.show_legend, bool);
        apply_val!("VISUAL_SHOW_GRID", self.visual.show_grid, bool);
        apply_val!("PHYSICS_IDEAL_DISTANCE", self.physics.ideal_distance, f64);
        apply_val!("PHYSICS_DAMPING", self.physics.damping, f32);
        apply_val!("PHYSICS_MAX_ITERATIONS", self.physics.max_iterations, usize);
        apply_val!("PHYSICS_GRAVITY", self.physics.gravity, f64);
        apply_val!(
            "INTERACTION_DOUBLE_CLICK_MS",
            self.interaction.double_click_ms,
            u64
        );
        apply_val!(
            "INTERACTION_PAN_SENSITIVITY",
            self.interaction.pan_sensitivity,
            f64
        );
        apply_val!("INTERACTION_ZOOM_FACTOR", self.interaction.zoom_factor, f64);
        apply_val!(
            "DISPLAY_SHOW_STATUS_BAR",
            self.display.show_status_bar,
            bool
        );
        if let Ok(s) = env::var("GRAF_DISPLAY_STATUS_FORMAT") {
            self.display.status_format = Some(s);
        }
        apply_enum!("DISPLAY_BORDER_STYLE", self.display.border_style);
        if let Ok(s) = env::var("GRAF_DISPLAY_BORDER_TITLE") {
            self.display.border_title = s;
        }
        if let Ok(s) = env::var("GRAF_FILTER_EXCLUDE_TAGS") {
            self.filter.exclude_tags = s.split(',').map(|s| s.trim().to_string()).collect();
        }
        if let Ok(s) = env::var("GRAF_FILTER_EXCLUDE_PATTERNS") {
            self.filter.exclude_patterns = s.split(',').map(|s| s.trim().to_string()).collect();
        }
        apply_val!("FILTER_MIN_LINKS", self.filter.min_links, usize);
        apply_val!("FILTER_MAX_NODES", self.filter.max_nodes, usize);
        apply_enum!("LEGEND_POSITION", self.legend.position);
        apply_val!("LEGEND_MAX_ITEMS", self.legend.max_items, usize);
    }
}
