use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(
    name = "graf",
    about = "Terminal force-directed graph visualizer for markdown notes"
)]
pub struct Cli {
    /// Directory to scan for markdown files (default: current directory)
    #[arg(short, long)]
    pub dir: Option<PathBuf>,

    /// Path to config file (default: XDG config dir)
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    /// Theme preset to use
    #[arg(long)]
    pub theme: Option<String>,

    /// Maximum number of nodes to display
    #[arg(long)]
    pub max_nodes: Option<usize>,

    /// Exclude files matching glob pattern (repeatable)
    #[arg(long)]
    pub exclude: Option<Vec<String>>,

    /// Exclude tags (comma-separated)
    #[arg(long)]
    pub exclude_tags: Option<String>,

    /// Node color mode
    #[arg(long)]
    pub node_color_mode: Option<String>,

    /// Edge color mode
    #[arg(long)]
    pub edge_color_mode: Option<String>,

    /// Label display mode
    #[arg(long)]
    pub label_mode: Option<String>,

    /// Show all labels
    #[arg(long)]
    pub labels: bool,

    /// Hide status bar
    #[arg(long)]
    pub no_status: bool,

    /// Show grid
    #[arg(long)]
    pub grid: bool,

    /// Hide minimap
    #[arg(long)]
    pub no_minimap: bool,

    /// Hide legend
    #[arg(long)]
    pub no_legend: bool,

    /// Background style
    #[arg(long)]
    pub background: Option<String>,

    /// Border style for overlays
    #[arg(long)]
    pub border_style: Option<String>,

    /// Editor command to open files
    #[arg(long)]
    pub editor: Option<String>,
}
