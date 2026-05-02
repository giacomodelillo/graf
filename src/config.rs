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
    TokyoNight,
    CatppuccinMocha,
    CatppuccinLatte,
    Onedark,
    Gruvbox,
    Dracula,
    Nord,
    RosePine,
    Everforest,
    Kanagawa,
    GruvboxLight,
    GitHubLight,
    OneLight,
}

impl FromStr for Theme {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "default" => Ok(Theme::Default),
            "tokyo_night" | "tokyonight" => Ok(Theme::TokyoNight),
            "catppuccin_mocha" | "catppuccinmocha" => Ok(Theme::CatppuccinMocha),
            "onedark" => Ok(Theme::Onedark),
            "gruvbox" => Ok(Theme::Gruvbox),
            "dracula" => Ok(Theme::Dracula),
            "nord" => Ok(Theme::Nord),
            "catppuccin_latte" | "catppuccinlatte" => Ok(Theme::CatppuccinLatte),
            "rose_pine" | "rosepine" => Ok(Theme::RosePine),
            "everforest" => Ok(Theme::Everforest),
            "kanagawa" => Ok(Theme::Kanagawa),
            "gruvbox_light" | "gruvboxlight" => Ok(Theme::GruvboxLight),
            "github_light" | "githublight" => Ok(Theme::GitHubLight),
            "one_light" | "onelight" => Ok(Theme::OneLight),
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
            "link_count" | "linkcount" => Ok(NodeColorMode::LinkCount),
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
            "link_count" | "linkcount" => Ok(NodeSizeMode::LinkCount),
            _ => Err(format!("Unknown node_size_mode: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CanvasMarker {
    #[default]
    Braille,
    HalfBlock,
    Dot,
}

impl FromStr for CanvasMarker {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "braille" => Ok(CanvasMarker::Braille),
            "half_block" | "halfblock" => Ok(CanvasMarker::HalfBlock),
            "dot" => Ok(CanvasMarker::Dot),
            _ => Err(format!("Unknown canvas_marker: {}", s)),
        }
    }
}

impl From<CanvasMarker> for ratatui::symbols::Marker {
    fn from(m: CanvasMarker) -> Self {
        match m {
            CanvasMarker::Braille => ratatui::symbols::Marker::Braille,
            CanvasMarker::HalfBlock => ratatui::symbols::Marker::HalfBlock,
            CanvasMarker::Dot => ratatui::symbols::Marker::Dot,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum NodeShape {
    #[default]
    Circle,
    Square,
    Diamond,
}

impl FromStr for NodeShape {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "circle" => Ok(NodeShape::Circle),
            "square" => Ok(NodeShape::Square),
            "diamond" => Ok(NodeShape::Diamond),
            _ => Err(format!("Unknown node_shape: {}", s)),
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
            "top_right" | "topright" => Ok(LegendPosition::TopRight),
            "top_left" | "topleft" => Ok(LegendPosition::TopLeft),
            "bottom_right" | "bottomright" => Ok(LegendPosition::BottomRight),
            "bottom_left" | "bottomleft" => Ok(LegendPosition::BottomLeft),
            _ => Err(format!("Unknown legend position: {}", s)),
        }
    }
}

impl Default for LegendPosition {
    fn default() -> Self {
        Self::TopRight
    }
}

fn parse_hex_color(s: &str) -> Option<Color> {
    let s = s.strip_prefix('#')?;
    if s.len() == 6 {
        let r = u8::from_str_radix(&s[0..2], 16).ok()?;
        let g = u8::from_str_radix(&s[2..4], 16).ok()?;
        let b = u8::from_str_radix(&s[4..6], 16).ok()?;
        Some(Color::Rgb(r, g, b))
    } else {
        None
    }
}

fn deserialize_optional_color<'de, D>(deserializer: D) -> Result<Option<Color>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;
    match opt {
        None => Ok(None),
        Some(s) => parse_hex_color(&s)
            .map(Some)
            .ok_or_else(|| serde::de::Error::custom(format!("invalid hex color: {}", s))),
    }
}

#[derive(Debug, Clone, Default)]
pub struct ColorOverrides {
    pub node_color: Option<Color>,
    pub edge_color: Option<Color>,
    pub label_color: Option<Color>,
    pub selection_ring_color: Option<Color>,
    pub border_color: Option<Color>,
    pub title_color: Option<Color>,
    pub grid_color: Option<Color>,
    pub legend_text_color: Option<Color>,
    pub status_bar_color: Option<Color>,
    pub background_color: Option<Color>,
}

impl serde::Serialize for ColorOverrides {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("ColorOverrides", 10)?;
        fn fmt_color(c: &Color) -> String {
            if let Color::Rgb(r, g, b) = c {
                format!("#{:02x}{:02x}{:02x}", r, g, b)
            } else {
                format!("{:?}", c)
            }
        }
        if let Some(ref v) = self.node_color {
            s.serialize_field("node_color", &fmt_color(v))?;
        }
        if let Some(ref v) = self.edge_color {
            s.serialize_field("edge_color", &fmt_color(v))?;
        }
        if let Some(ref v) = self.label_color {
            s.serialize_field("label_color", &fmt_color(v))?;
        }
        if let Some(ref v) = self.selection_ring_color {
            s.serialize_field("selection_ring_color", &fmt_color(v))?;
        }
        if let Some(ref v) = self.border_color {
            s.serialize_field("border_color", &fmt_color(v))?;
        }
        if let Some(ref v) = self.title_color {
            s.serialize_field("title_color", &fmt_color(v))?;
        }
        if let Some(ref v) = self.grid_color {
            s.serialize_field("grid_color", &fmt_color(v))?;
        }
        if let Some(ref v) = self.legend_text_color {
            s.serialize_field("legend_text_color", &fmt_color(v))?;
        }
        if let Some(ref v) = self.status_bar_color {
            s.serialize_field("status_bar_color", &fmt_color(v))?;
        }
        if let Some(ref v) = self.background_color {
            s.serialize_field("background_color", &fmt_color(v))?;
        }
        s.end()
    }
}

impl<'de> serde::Deserialize<'de> for ColorOverrides {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct ColorOverridesRaw {
            #[serde(default, deserialize_with = "deserialize_optional_color")]
            node_color: Option<Color>,
            #[serde(default, deserialize_with = "deserialize_optional_color")]
            edge_color: Option<Color>,
            #[serde(default, deserialize_with = "deserialize_optional_color")]
            label_color: Option<Color>,
            #[serde(default, deserialize_with = "deserialize_optional_color")]
            selection_ring_color: Option<Color>,
            #[serde(default, deserialize_with = "deserialize_optional_color")]
            border_color: Option<Color>,
            #[serde(default, deserialize_with = "deserialize_optional_color")]
            title_color: Option<Color>,
            #[serde(default, deserialize_with = "deserialize_optional_color")]
            grid_color: Option<Color>,
            #[serde(default, deserialize_with = "deserialize_optional_color")]
            legend_text_color: Option<Color>,
            #[serde(default, deserialize_with = "deserialize_optional_color")]
            status_bar_color: Option<Color>,
            #[serde(default, deserialize_with = "deserialize_optional_color")]
            background_color: Option<Color>,
        }
        let raw = ColorOverridesRaw::deserialize(deserializer)?;
        Ok(ColorOverrides {
            node_color: raw.node_color,
            edge_color: raw.edge_color,
            label_color: raw.label_color,
            selection_ring_color: raw.selection_ring_color,
            border_color: raw.border_color,
            title_color: raw.title_color,
            grid_color: raw.grid_color,
            legend_text_color: raw.legend_text_color,
            status_bar_color: raw.status_bar_color,
            background_color: raw.background_color,
        })
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
    #[serde(default = "default_true")]
    pub show_minimap: bool,
    #[serde(default)]
    pub minimap_position: LegendPosition,
    #[serde(default = "default_minimap_width")]
    pub minimap_width: u16,
    #[serde(default = "default_minimap_height")]
    pub minimap_height: u16,
    #[serde(default)]
    pub canvas_marker: CanvasMarker,
    #[serde(default)]
    pub minimap_marker: CanvasMarker,
    #[serde(default)]
    pub node_shape: NodeShape,
    #[serde(default = "default_label_offset")]
    pub label_offset: f64,
    #[serde(default = "default_grid_divisions")]
    pub grid_divisions: usize,
    #[serde(default)]
    pub colors: ColorOverrides,
}

impl Default for VisualConfig {
    fn default() -> Self {
        Self {
            theme: Theme::Default,
            background: Background::Solid,
            node_color_mode: NodeColorMode::Folder,
            edge_color_mode: EdgeColorMode::Uniform,
            label_mode: LabelMode::default(),
            label_max_length: default_label_max(),
            node_size: default_node_size(),
            node_size_mode: NodeSizeMode::default(),
            edge_thickness: default_edge_thickness(),
            show_legend: default_true(),
            show_grid: false,
            show_minimap: false,
            minimap_position: LegendPosition::TopRight,
            minimap_width: default_minimap_width(),
            minimap_height: default_minimap_height(),
            canvas_marker: CanvasMarker::Braille,
            minimap_marker: CanvasMarker::HalfBlock,
            node_shape: NodeShape::default(),
            label_offset: default_label_offset(),
            grid_divisions: default_grid_divisions(),
            colors: ColorOverrides::default(),
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
    #[serde(default = "default_true")]
    pub cooling: bool,
    #[serde(default = "default_true")]
    pub prevent_overlapping: bool,
    #[serde(default = "default_timestep")]
    pub timestep: f64,
    #[serde(default = "default_thread_sleep_ms")]
    pub thread_sleep_ms: u64,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            ideal_distance: default_ideal_distance(),
            damping: default_damping(),
            max_iterations: default_max_iterations(),
            gravity: default_gravity(),
            cooling: default_true(),
            prevent_overlapping: default_true(),
            timestep: default_timestep(),
            thread_sleep_ms: default_thread_sleep_ms(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionConfig {
    #[serde(default = "default_double_click")]
    pub double_click_ms: u64,
    #[serde(default = "default_zoom_factor")]
    pub zoom_factor: f64,
    #[serde(default = "default_drag_sensitivity")]
    pub drag_sensitivity: f64,
    #[serde(default = "default_auto_fit_padding")]
    pub auto_fit_padding: f64,
    #[serde(default = "default_drag_scale")]
    pub drag_scale: f64,
}

impl Default for InteractionConfig {
    fn default() -> Self {
        Self {
            double_click_ms: default_double_click(),
            zoom_factor: default_zoom_factor(),
            drag_sensitivity: default_drag_sensitivity(),
            auto_fit_padding: default_auto_fit_padding(),
            drag_scale: default_drag_scale(),
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
            position: LegendPosition::BottomRight,
            max_items: 100,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    #[serde(default = "default_search_max_results")]
    pub max_results: usize,
    #[serde(default = "default_search_max_visible")]
    pub max_visible: usize,
    #[serde(default = "default_search_popup_width")]
    pub popup_width: u16,
    #[serde(default = "default_search_popup_y")]
    pub popup_y: u16,
    #[serde(default = "default_search_cursor_glyph")]
    pub cursor_glyph: String,
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            max_results: default_search_max_results(),
            max_visible: default_search_max_visible(),
            popup_width: default_search_popup_width(),
            popup_y: default_search_popup_y(),
            cursor_glyph: default_search_cursor_glyph(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorConfig {
    #[serde(default)]
    pub command: String,
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            command: String::new(),
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
    #[serde(default)]
    pub search: SearchConfig,
    #[serde(default)]
    pub editor: EditorConfig,
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
            search: SearchConfig::default(),
            editor: EditorConfig::default(),
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
fn default_zoom_factor() -> f64 {
    1.15
}
fn default_drag_sensitivity() -> f64 {
    1.0
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
fn default_minimap_width() -> u16 {
    24
}
fn default_minimap_height() -> u16 {
    12
}
fn default_label_offset() -> f64 {
    4.0
}
fn default_grid_divisions() -> usize {
    10
}
fn default_timestep() -> f64 {
    0.016
}
fn default_thread_sleep_ms() -> u64 {
    16
}
fn default_auto_fit_padding() -> f64 {
    1.4
}
fn default_drag_scale() -> f64 {
    200.0
}
fn default_search_max_results() -> usize {
    20
}
fn default_search_max_visible() -> usize {
    10
}
fn default_search_popup_width() -> u16 {
    50
}
fn default_search_popup_y() -> u16 {
    3
}
fn default_search_cursor_glyph() -> String {
    "▎".to_string()
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
    pub minimap_border_color: Color,
    pub minimap_viewport_color: Color,
    pub minimap_bg_color: Option<Color>,
}

impl GrafConfig {
    pub fn config_path() -> anyhow::Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "graf", "graf")
            .ok_or_else(|| anyhow::anyhow!("no home dir"))?;
        Ok(proj_dirs.config_dir().join("config.toml"))
    }

    pub fn theme_colors(&self) -> ThemeColors {
        let mut colors = match self.visual.theme {
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
                minimap_border_color: Color::DarkGray,
                minimap_viewport_color: Color::White,
                minimap_bg_color: None,
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
                minimap_border_color: Color::Rgb(86, 95, 137),
                minimap_viewport_color: Color::Rgb(255, 255, 255),
                minimap_bg_color: Some(Color::Rgb(26, 27, 38)),
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
                minimap_border_color: Color::Rgb(102, 92, 84),
                minimap_viewport_color: Color::Rgb(251, 241, 199),
                minimap_bg_color: Some(Color::Rgb(40, 40, 40)),
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
                minimap_border_color: Color::Rgb(98, 114, 164),
                minimap_viewport_color: Color::Rgb(255, 255, 255),
                minimap_bg_color: Some(Color::Rgb(40, 42, 54)),
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
                minimap_border_color: Color::Rgb(67, 76, 94),
                minimap_viewport_color: Color::Rgb(236, 239, 244),
                minimap_bg_color: Some(Color::Rgb(46, 52, 64)),
            },
            Theme::CatppuccinLatte => ThemeColors {
                node_colors: vec![
                    Color::Rgb(239, 159, 118), // peach
                    Color::Rgb(244, 184, 228), // pink
                    Color::Rgb(137, 180, 250), // blue
                    Color::Rgb(116, 199, 236), // sky
                    Color::Rgb(166, 227, 161), // green
                    Color::Rgb(249, 226, 175), // yellow
                    Color::Rgb(235, 160, 172), // red
                    Color::Rgb(180, 190, 254), // lavender
                ],
                edge_color: Color::Rgb(76, 79, 105),
                border_color: Color::Rgb(76, 79, 105),
                title_color: Color::Rgb(137, 180, 250),
                label_color: Color::Rgb(76, 79, 105),
                legend_text_color: Color::Rgb(76, 79, 105),
                legend_border_color: Color::Rgb(76, 79, 105),
                selected_indicator_color: Color::Rgb(76, 79, 105),
                grid_color: Color::Rgb(209, 213, 228),
                background_color: match self.visual.background {
                    Background::Transparent => None,
                    Background::Solid => Some(Color::Rgb(239, 241, 245)),
                },
                status_bar_color: Color::Rgb(76, 79, 105),
                minimap_border_color: Color::Rgb(76, 79, 105),
                minimap_viewport_color: Color::Rgb(76, 79, 105),
                minimap_bg_color: Some(Color::Rgb(239, 241, 245)),
            },
            Theme::RosePine => ThemeColors {
                node_colors: vec![
                    Color::Rgb(180, 142, 173), // love
                    Color::Rgb(234, 154, 151), // gold
                    Color::Rgb(156, 207, 216), // pine
                    Color::Rgb(246, 193, 119), // foam
                    Color::Rgb(155, 138, 221), // iris
                    Color::Rgb(235, 111, 146), // rose
                    Color::Rgb(159, 188, 198), // water
                    Color::Rgb(209, 193, 168), // sand
                ],
                edge_color: Color::Rgb(102, 110, 129),
                border_color: Color::Rgb(102, 110, 129),
                title_color: Color::Rgb(156, 207, 216),
                label_color: Color::Rgb(87, 82, 121),
                legend_text_color: Color::Rgb(87, 82, 121),
                legend_border_color: Color::Rgb(102, 110, 129),
                selected_indicator_color: Color::Rgb(87, 82, 121),
                grid_color: Color::Rgb(57, 53, 82),
                background_color: match self.visual.background {
                    Background::Transparent => None,
                    Background::Solid => Some(Color::Rgb(40, 37, 61)),
                },
                status_bar_color: Color::Rgb(102, 110, 129),
                minimap_border_color: Color::Rgb(102, 110, 129),
                minimap_viewport_color: Color::Rgb(87, 82, 121),
                minimap_bg_color: Some(Color::Rgb(40, 37, 61)),
            },
            Theme::Everforest => ThemeColors {
                node_colors: vec![
                    Color::Rgb(255, 215, 89),  // yellow
                    Color::Rgb(255, 143, 105), // red
                    Color::Rgb(129, 204, 165), // green
                    Color::Rgb(100, 200, 218), // aqua
                    Color::Rgb(150, 205, 255), // blue
                    Color::Rgb(220, 150, 255), // purple
                    Color::Rgb(255, 180, 120), // orange
                    Color::Rgb(200, 230, 150), // lime
                ],
                edge_color: Color::Rgb(95, 120, 102),
                border_color: Color::Rgb(95, 120, 102),
                title_color: Color::Rgb(129, 204, 165),
                label_color: Color::Rgb(60, 76, 67),
                legend_text_color: Color::Rgb(60, 76, 67),
                legend_border_color: Color::Rgb(95, 120, 102),
                selected_indicator_color: Color::Rgb(60, 76, 67),
                grid_color: Color::Rgb(40, 50, 45),
                background_color: match self.visual.background {
                    Background::Transparent => None,
                    Background::Solid => Some(Color::Rgb(30, 38, 34)),
                },
                status_bar_color: Color::Rgb(95, 120, 102),
                minimap_border_color: Color::Rgb(95, 120, 102),
                minimap_viewport_color: Color::Rgb(60, 76, 67),
                minimap_bg_color: Some(Color::Rgb(30, 38, 34)),
            },
            Theme::Kanagawa => ThemeColors {
                node_colors: vec![
                    Color::Rgb(147, 191, 254), // blue
                    Color::Rgb(255, 158, 181), // pink
                    Color::Rgb(203, 166, 247), // purple
                    Color::Rgb(137, 180, 130), // green
                    Color::Rgb(247, 234, 168), // yellow
                    Color::Rgb(255, 173, 130), // peach
                    Color::Rgb(125, 196, 228), // sky
                    Color::Rgb(242, 205, 205), // red
                ],
                edge_color: Color::Rgb(95, 115, 135),
                border_color: Color::Rgb(95, 115, 135),
                title_color: Color::Rgb(147, 191, 254),
                label_color: Color::Rgb(98, 114, 164),
                legend_text_color: Color::Rgb(98, 114, 164),
                legend_border_color: Color::Rgb(95, 115, 135),
                selected_indicator_color: Color::Rgb(98, 114, 164),
                grid_color: Color::Rgb(34, 40, 62),
                background_color: match self.visual.background {
                    Background::Transparent => None,
                    Background::Solid => Some(Color::Rgb(26, 30, 48)),
                },
                status_bar_color: Color::Rgb(95, 115, 135),
                minimap_border_color: Color::Rgb(95, 115, 135),
                minimap_viewport_color: Color::Rgb(98, 114, 164),
                minimap_bg_color: Some(Color::Rgb(26, 30, 48)),
            },
            Theme::GruvboxLight => ThemeColors {
                node_colors: vec![
                    Color::Rgb(204, 36, 29),   // red
                    Color::Rgb(152, 151, 26),   // green
                    Color::Rgb(215, 153, 33),   // yellow
                    Color::Rgb(69, 133, 136),   // blue
                    Color::Rgb(177, 98, 134),   // purple
                    Color::Rgb(104, 157, 106),  // aqua
                    Color::Rgb(214, 93, 14),    // orange
                    Color::Rgb(168, 153, 132),  // gray
                ],
                edge_color: Color::Rgb(60, 56, 54),
                border_color: Color::Rgb(60, 56, 54),
                title_color: Color::Rgb(69, 133, 136),
                label_color: Color::Rgb(60, 56, 54),
                legend_text_color: Color::Rgb(60, 56, 54),
                legend_border_color: Color::Rgb(60, 56, 54),
                selected_indicator_color: Color::Rgb(60, 56, 54),
                grid_color: Color::Rgb(210, 195, 160),
                background_color: match self.visual.background {
                    Background::Transparent => None,
                    Background::Solid => Some(Color::Rgb(251, 241, 199)),
                },
                status_bar_color: Color::Rgb(60, 56, 54),
                minimap_border_color: Color::Rgb(60, 56, 54),
                minimap_viewport_color: Color::Rgb(60, 56, 54),
                minimap_bg_color: Some(Color::Rgb(251, 241, 199)),
            },
            Theme::GitHubLight => ThemeColors {
                node_colors: vec![
                    Color::Rgb(9, 105, 218),   // blue
                    Color::Rgb(26, 127, 55),   // green
                    Color::Rgb(191, 135, 0),   // yellow
                    Color::Rgb(207, 34, 46),   // red
                    Color::Rgb(130, 80, 223),  // purple
                    Color::Rgb(31, 136, 61),   // dark green
                    Color::Rgb(33, 139, 255),  // light blue
                    Color::Rgb(101, 109, 118), // gray
                ],
                edge_color: Color::Rgb(31, 35, 40),
                border_color: Color::Rgb(31, 35, 40),
                title_color: Color::Rgb(9, 105, 218),
                label_color: Color::Rgb(31, 35, 40),
                legend_text_color: Color::Rgb(31, 35, 40),
                legend_border_color: Color::Rgb(31, 35, 40),
                selected_indicator_color: Color::Rgb(31, 35, 40),
                grid_color: Color::Rgb(208, 215, 222),
                background_color: match self.visual.background {
                    Background::Transparent => None,
                    Background::Solid => Some(Color::Rgb(255, 255, 255)),
                },
                status_bar_color: Color::Rgb(31, 35, 40),
                minimap_border_color: Color::Rgb(31, 35, 40),
                minimap_viewport_color: Color::Rgb(31, 35, 40),
                minimap_bg_color: Some(Color::Rgb(255, 255, 255)),
            },
            Theme::OneLight => ThemeColors {
                node_colors: vec![
                    Color::Rgb(64, 120, 242),  // blue
                    Color::Rgb(103, 161, 68),   // green
                    Color::Rgb(192, 139, 48),   // yellow
                    Color::Rgb(224, 82, 82),    // red
                    Color::Rgb(176, 111, 224),  // purple
                    Color::Rgb(80, 161, 79),    // dark green
                    Color::Rgb(1, 132, 188),    // cyan
                    Color::Rgb(160, 161, 167),  // gray
                ],
                edge_color: Color::Rgb(56, 58, 66),
                border_color: Color::Rgb(56, 58, 66),
                title_color: Color::Rgb(64, 120, 242),
                label_color: Color::Rgb(56, 58, 66),
                legend_text_color: Color::Rgb(56, 58, 66),
                legend_border_color: Color::Rgb(56, 58, 66),
                selected_indicator_color: Color::Rgb(56, 58, 66),
                grid_color: Color::Rgb(230, 230, 230),
                background_color: match self.visual.background {
                    Background::Transparent => None,
                    Background::Solid => Some(Color::Rgb(250, 250, 250)),
                },
                status_bar_color: Color::Rgb(56, 58, 66),
                minimap_border_color: Color::Rgb(56, 58, 66),
                minimap_viewport_color: Color::Rgb(56, 58, 66),
                minimap_bg_color: Some(Color::Rgb(250, 250, 250)),
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
                minimap_border_color: Color::Rgb(108, 112, 134),
                minimap_viewport_color: Color::Rgb(205, 214, 244),
                minimap_bg_color: Some(Color::Rgb(30, 30, 46)),
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
                minimap_border_color: Color::Rgb(92, 99, 112),
                minimap_viewport_color: Color::Rgb(220, 223, 228),
                minimap_bg_color: Some(Color::Rgb(40, 44, 52)),
            },
        };

        if let Some(ref c) = self.visual.colors.node_color {
            colors.node_colors = vec![*c];
        }
        if let Some(c) = self.visual.colors.edge_color {
            colors.edge_color = c;
        }
        if let Some(c) = self.visual.colors.label_color {
            colors.label_color = c;
        }
        if let Some(c) = self.visual.colors.selection_ring_color {
            colors.selected_indicator_color = c;
        }
        if let Some(c) = self.visual.colors.border_color {
            colors.border_color = c;
            colors.legend_border_color = c;
            colors.minimap_border_color = c;
        }
        if let Some(c) = self.visual.colors.title_color {
            colors.title_color = c;
        }
        if let Some(c) = self.visual.colors.grid_color {
            colors.grid_color = c;
        }
        if let Some(c) = self.visual.colors.legend_text_color {
            colors.legend_text_color = c;
        }
        if let Some(c) = self.visual.colors.status_bar_color {
            colors.status_bar_color = c;
        }
        if let Some(c) = self.visual.colors.background_color {
            colors.background_color = Some(c);
            colors.minimap_bg_color = Some(c);
        }

        colors
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

    pub fn expand_status(
        &self,
        files: usize,
        links: usize,
        selected: Option<&str>,
        viewport_size_pct: Option<f64>,
        viewport_ratio: Option<f64>,
    ) -> String {
        let fmt = self
            .display
            .status_format
            .as_deref()
            .unwrap_or("Files: {files} | Links: {links} | Selected: {selected}");
        let fmt = fmt.replace("{files}", &files.to_string());
        let fmt = fmt.replace("{links}", &links.to_string());
        let fmt = fmt.replace("{selected}", selected.unwrap_or("none"));
        let fmt = fmt.replace(
            "{date}",
            &chrono::Local::now().format("%Y-%m-%d").to_string(),
        );
        let fmt = fmt.replace(
            "{time}",
            &chrono::Local::now().format("%H:%M:%S").to_string(),
        );
        let fmt = fmt.replace(
            "{size}",
            &format!("{:.0}%", viewport_size_pct.unwrap_or(0.0).clamp(0.0, 100.0)),
        );
        let fmt = fmt.replace("{ratio}", &format!("{:.1}x", viewport_ratio.unwrap_or(1.0)));
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
        errs
    }

    pub fn load_from_path(path: Option<PathBuf>) -> (Self, Vec<String>, bool) {
        let mut config = Self::default();
        let mut errors = Vec::new();
        let mut created = false;

        if let Some(path) = path {
            if path.exists() {
                match fs::read_to_string(&path) {
                    Ok(content) => match toml::from_str::<GrafConfig>(&content) {
                        Ok(loaded) => config = loaded,
                        Err(e) => errors.push(format!("Invalid config TOML: {}", e)),
                    },
                    Err(e) => errors.push(format!("Cannot read config file: {}", e)),
                }
            } else if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
                let _ = fs::write(&path, generate_default_toml());
                created = true;
            }
        }

        config.apply_env_overrides();

        (config, errors, created)
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
        apply_val!("VISUAL_SHOW_MINIMAP", self.visual.show_minimap, bool);
        apply_enum!("VISUAL_MINIMAP_POSITION", self.visual.minimap_position);
        apply_val!("VISUAL_MINIMAP_WIDTH", self.visual.minimap_width, u16);
        apply_val!("VISUAL_MINIMAP_HEIGHT", self.visual.minimap_height, u16);
        apply_enum!("VISUAL_CANVAS_MARKER", self.visual.canvas_marker);
        apply_enum!("VISUAL_MINIMAP_MARKER", self.visual.minimap_marker);
        apply_enum!("VISUAL_NODE_SHAPE", self.visual.node_shape);
        apply_val!("VISUAL_LABEL_OFFSET", self.visual.label_offset, f64);
        apply_val!("VISUAL_GRID_DIVISIONS", self.visual.grid_divisions, usize);
        apply_val!("PHYSICS_IDEAL_DISTANCE", self.physics.ideal_distance, f64);
        apply_val!("PHYSICS_DAMPING", self.physics.damping, f32);
        apply_val!("PHYSICS_MAX_ITERATIONS", self.physics.max_iterations, usize);
        apply_val!("PHYSICS_GRAVITY", self.physics.gravity, f64);
        apply_val!("PHYSICS_COOLING", self.physics.cooling, bool);
        apply_val!(
            "PHYSICS_PREVENT_OVERLAPPING",
            self.physics.prevent_overlapping,
            bool
        );
        apply_val!("PHYSICS_TIMESTEP", self.physics.timestep, f64);
        apply_val!("PHYSICS_THREAD_SLEEP_MS", self.physics.thread_sleep_ms, u64);
        apply_val!(
            "INTERACTION_DOUBLE_CLICK_MS",
            self.interaction.double_click_ms,
            u64
        );
        apply_val!("INTERACTION_ZOOM_FACTOR", self.interaction.zoom_factor, f64);
        apply_val!(
            "INTERACTION_DRAG_SENSITIVITY",
            self.interaction.drag_sensitivity,
            f64
        );
        apply_val!(
            "INTERACTION_AUTO_FIT_PADDING",
            self.interaction.auto_fit_padding,
            f64
        );
        apply_val!("INTERACTION_DRAG_SCALE", self.interaction.drag_scale, f64);
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
        apply_val!("SEARCH_MAX_RESULTS", self.search.max_results, usize);
        apply_val!("SEARCH_MAX_VISIBLE", self.search.max_visible, usize);
        apply_val!("SEARCH_POPUP_WIDTH", self.search.popup_width, u16);
        apply_val!("SEARCH_POPUP_Y", self.search.popup_y, u16);
        if let Ok(s) = env::var("GRAF_SEARCH_CURSOR_GLYPH") {
            self.search.cursor_glyph = s;
        }
        if let Ok(s) = env::var("GRAF_EDITOR_COMMAND") {
            self.editor.command = s;
        }
    }
}

fn generate_default_toml() -> String {
    r###"# graf configuration
# Edit values below to customize. Restart graf to apply changes.

[visual]
# Theme: "default", "tokyo_night", "catppuccin_mocha", "catppuccin_latte", "onedark", "gruvbox", "dracula", "nord", "rose_pine", "everforest", "kanagawa"
theme = "default"
# Background: "transparent", "solid"
background = "solid"
# Node color mode: "tag", "folder", "link_count", "uniform"
node_color_mode = "folder"
# Edge color mode: "source", "target", "uniform"
edge_color_mode = "uniform"
# Label display: "selected", "neighbors", "all", "none"
label_mode = "selected"
# Max label length (1-60)
label_max_length = 20
# Base node radius (1.0-5.0)
node_size = 2.0
# Node size mode: "fixed", "link_count"
node_size_mode = "fixed"
# Edge thickness (1-3)
edge_thickness = 1
show_legend = true
show_grid = false
show_minimap = false
# Minimap corner: "top_right", "top_left", "bottom_right", "bottom_left"
minimap_position = "top_right"
minimap_width = 24
minimap_height = 12
# Main canvas marker: "braille", "half_block", "dot"
canvas_marker = "braille"
# Minimap marker: "braille", "half_block", "dot"
minimap_marker = "half_block"
# Node shape: "circle", "square", "diamond"
node_shape = "circle"
# Distance between node center and label
label_offset = 4.0
# Number of grid lines per axis
grid_divisions = 10

# Color overrides (hex, applied on top of theme). Uncomment to override theme defaults.
[visual.colors]
# node_color = "#ff6600"
# edge_color = "#445566"
# label_color = "#aabbcc"
# selection_ring_color = "#ff00ff"
# border_color = "#334455"
# title_color = "#66ffcc"
# grid_color = "#222233"
# legend_text_color = "#ccddee"
# status_bar_color = "#556677"
# background_color = "#1a1a2e"

[physics]
# ideal_distance = 80.0
# damping = 0.95
# max_iterations = 800
# gravity = 0.01
# cooling = true
# prevent_overlapping = true
# timestep = 0.016
# thread_sleep_ms = 16

[interaction]
# double_click_ms = 300
# # Must be > 0
# zoom_factor = 1.15
# drag_sensitivity = 1.0
# auto_fit_padding = 1.4
# drag_scale = 200.0

[display]
# show_status_bar = true
# # Status format placeholders: {files} {links} {selected} {date} {time} {size} {ratio}
# # status_format = "{files} files | {links} links | {selected}"
# # Border: "plain", "rounded", "double", "none"
# border_style = "rounded"
# # Title placeholders: {cwd}
# border_title = "graf"

[filter]
# # exclude_tags = ["draft", "private"]
# # exclude_patterns = ["templates/", "private/"]
# min_links = 0
# max_nodes = 500

[legend]
# Position: "top_right", "top_left", "bottom_right", "bottom_left"
position = "bottom_right"
max_items = 100

[search]
max_results = 20
max_visible = 10
popup_width = 50
popup_y = 3
cursor_glyph = "▎"

[editor]
# Falls back to $EDITOR then vim if empty
# command = "nano"
"###.to_string()
}
