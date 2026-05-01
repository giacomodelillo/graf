# graf

A terminal-based force-directed graph visualizer for markdown wikilinks. Run `graf` in any directory and see your markdown files as an interactive, pannable, zoomable network graph.

## Installation

### From source

```bash
git clone <repo-url>
cd graf
cargo install --path .
```

### Dependencies

- Rust 1.75+ (Edition 2024)

## Usage

Navigate to any directory containing markdown files and run:

```bash
cd ~/docs/my-wiki
graf
```

`graf` recursively scans the current working directory for `*.md` and `*.mdx` files, parses `[[wikilinks]]` and YAML frontmatter tags, then renders a force-directed graph in your terminal.

### Opening files

Double-click a node or press `Enter` to open the linked file. The editor is read from the `EDITOR` environment variable (defaults to `vim`).

```bash
EDITOR=nvim graf
```

## Controls

### Keyboard

| Key | Action |
|-----|--------|
| `↑` `↓` `←` `→` | Pan the view |
| `+` `=` | Zoom in |
| `-` | Zoom out |
| `Enter` | Open selected file in editor |
| `a` | Auto-fit view to all nodes |
| `?` | Toggle help overlay |
| `q` / `Esc` | Quit |

### Mouse

| Action | Effect |
|--------|--------|
| Scroll wheel | Zoom in/out |
| Click & drag background | Pan view (natural: drag up → view goes down) |
| Click & drag node | Reposition node |
| Single click | Select node |
| Double-click | Open file in editor |
| Double-click empty area | Deselect node |

## Configuration

Config file location: `~/.config/graf/config.toml`

If the file doesn't exist, `graf` uses all defaults. Every setting is optional — only include the ones you want to override.

### Full example

```toml
[visual]
theme = "catppuccin-mocha"
background = "transparent"
node_color_mode = "tag"
edge_color_mode = "source"
label_mode = "selected"
label_max_length = 20
node_size = 2.0
node_size_mode = "link_count"
edge_thickness = 1
show_legend = true
show_grid = false

[physics]
ideal_distance = 80.0
repulsion_strength = 80.0
attraction_strength = 1.0
damping = 0.95
max_iterations = 800
gravity = 0.01

[interaction]
double_click_ms = 300
pan_sensitivity = 0.2
zoom_factor = 1.15

[display]
show_status_bar = true
status_format = "Files: {files} | Links: {links} | Selected: {selected}"
border_style = "rounded"
border_title = "graf"

[filter]
exclude_tags = ["draft", "private"]
exclude_patterns = ["*.bak", "archive/*"]
min_links = 0
max_nodes = 500

[legend]
position = "top_right"
max_items = 10
```

### `[visual]`

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `theme` | string | `"default"` | Color scheme preset (see **Themes** below) |
| `background` | string | `"transparent"` | `"transparent"` passes through terminal transparency, `"solid"` fills with theme background color |
| `node_color_mode` | string | `"tag"` | How nodes are colored: `"tag"` by primary tag, `"link_count"` by number of connections, `"uniform"` single color for all |
| `edge_color_mode` | string | `"source"` | Edge color: `"source"` matches source node, `"target"` matches target node, `"uniform"` single color |
| `label_mode` | string | `"selected"` | Which labels to show: `"selected"`, `"neighbors"`, `"all"`, `"none"` |
| `label_max_length` | int | `20` | Max characters for title labels (1–60) |
| `node_size` | float | `2.0` | Base node radius (1–5) |
| `node_size_mode` | string | `"fixed"` | `"fixed"` constant size, `"link_count"` scales with number of connections |
| `edge_thickness` | int | `1` | Line thickness for edges (1–3) |
| `show_legend` | bool | `true` | Show tag legend |
| `show_grid` | bool | `false` | Show background grid overlay |

### `[physics]`

Controls the force-directed layout simulation.

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `ideal_distance` | float | `80.0` | Target distance between connected nodes |
| `repulsion_strength` | float | `80.0` | How strongly nodes push each other apart |
| `attraction_strength` | float | `1.0` | How strongly connected nodes pull together |
| `damping` | float | `0.95` | Velocity damping per step (lower = faster settling) |
| `max_iterations` | int | `800` | Maximum simulation steps before stopping |
| `gravity` | float | `0.01` | Center pull to prevent nodes from drifting off-screen |

### `[interaction]`

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `double_click_ms` | int | `300` | Time window in milliseconds to register a double-click |
| `pan_sensitivity` | float | `0.2` | Pan speed multiplier for keyboard and mouse |
| `zoom_factor` | float | `1.15` | Zoom multiplier per scroll or key press |

### `[display]`

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `show_status_bar` | bool | `true` | Show the status bar at the bottom |
| `status_format` | string | `"Files: {files} \| Links: {links} \| Selected: {selected}"` | Status bar text template. Variables: `{files}`, `{links}`, `{selected}` |
| `border_style` | string | `"rounded"` | Border style: `"plain"`, `"rounded"`, `"double"`, `"none"` |
| `border_title` | string | `"graf"` | Window title. Supports `{cwd}` variable (current directory name) |

### `[filter]`

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `exclude_tags` | array | `[]` | Tags to exclude from the graph |
| `exclude_patterns` | array | `[]` | Glob patterns for files to exclude |
| `min_links` | int | `0` | Only show nodes with at least this many connections |
| `max_nodes` | int | `500` | Maximum nodes to display (`0` = unlimited) |

### `[legend]`

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `position` | string | `"top_right"` | Legend position: `"top_right"`, `"top_left"`, `"bottom_right"`, `"bottom_left"` |
| `max_items` | int | `10` | Maximum tag groups to show in the legend |

## Themes

### Presets

| Value | Description |
|-------|-------------|
| `"default"` | Inherits colors from your terminal's color scheme. No hardcoded palette. |
| `"tokyo-night"` | Tokyo Night (dark) palette |
| `"gruvbox"` | Gruvbox Dark palette |
| `"dracula"` | Dracula palette |
| `"nord"` | Nord Frost palette |
| `"catppuccin-mocha"` | Catppuccin Mocha palette |
| `"onedark"` | One Dark palette |

### Default theme behavior

When `theme = "default"`, nodes use `Color::Reset` which maps to your terminal's default foreground color. The legend and labels use `Gray` and `DarkGray` for contrast. This mode is designed to blend seamlessly with your existing terminal colorscheme.

### Background modes

- `"transparent"` — The canvas background is not drawn, allowing your terminal's background (including transparency effects) to show through.
- `"solid"` — The canvas background is filled with the theme's configured background color. Only relevant when using a non-default theme.

## How it works

### File scanning

`graf` starts at the current working directory and recursively collects all `*.md` and `*.mdx` files. Files matching patterns in `exclude_patterns` are skipped. Up to `max_nodes` files are loaded.

### Link parsing

For each file, `graf` extracts:

1. **Wikilinks** — `[[Note Title]]` or `[[Note Title|Display Text]]` patterns from the file content
2. **Title** — From YAML frontmatter (`title: "My Note"`) or the first `# heading`
3. **Tags** — From YAML frontmatter (`tags: [tag1, tag2]`)

Links are resolved by matching wikilink text (case-insensitive) against file titles. Unresolved links are silently ignored.

### Color assignment

- **Tag mode**: Each unique tag across all files is assigned a unique color using golden-ratio distributed HSL values across the full 360° hue spectrum. No two tags ever share the same color.
- **Link count mode**: Colors are interpolated across the theme's node palette based on how many connections each node has.
- **Multi-tag nodes**: The primary (first) tag determines the main node color. Additional tags appear as small colored dots orbiting the node.

### Physics

A force-directed layout runs on a background thread at ~60fps using `fdg-sim`. Repulsive forces push all nodes apart, spring forces pull connected nodes together, and optional gravity prevents drift. The simulation auto-stops when total energy drops below a threshold.

## Terminal management

When opening a file in an external editor, `graf` suspends its terminal session:

1. Disables raw mode
2. Leaves the alternate screen
3. Spawns the editor with normal terminal state
4. On editor exit, re-enters alternate screen and re-enables raw mode

The `TerminalGuard` RAII struct ensures the terminal is always properly restored on exit, including on panic or Ctrl+C.

## Tech stack

| Crate | Purpose |
|-------|---------|
| `ratatui` | Terminal UI framework |
| `crossterm` | Terminal raw mode, mouse capture, alternate screen |
| `fdg-sim` | Force-directed graph simulation |
| `petgraph` | Graph data structures |
| `regex` | Wikilink and frontmatter parsing |
| `toml` + `serde` | Configuration loading |
| `glob` | File discovery |

## Project structure

```
graf/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs          # Entry point, terminal setup, event loop
    ├── app.rs           # App state management, graph initialization
    ├── config.rs        # Config loading, 7 theme presets, color palettes
    ├── linker.rs        # File scanning, wikilink extraction, frontmatter parsing
    ├── ui.rs            # Help overlay rendering
    └── graph/
        ├── mod.rs       # GraphState, ForceGraph construction
        ├── render.rs    # Canvas drawing (nodes, edges, labels, legend, grid)
        ├── input.rs     # Keyboard and mouse event handling
        ├── viewport.rs  # Pan, zoom, screen-to-world conversion, hit-testing
        └── physics.rs   # Background thread for force simulation
```

## License

MIT
