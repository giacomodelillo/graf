# graf

A terminal-based force-directed graph visualizer for markdown wikilinks. Run `graf` in any directory and see your markdown files as an interactive, pannable, zoomable network graph.

> Part of **[clin-rs](https://github.com/reekta92/clin-rs)**

<!-- TODO: Add screenshot -->
<!-- ![graf demo](assets/demo.gif) -->

## Features

**Layout & Physics**

- **Force-directed layout** — runs on a background thread at ~60fps using `fdg-sim`. Repulsive forces push all nodes apart, spring forces pull connected nodes together, and optional gravity prevents drift. The simulation auto-cools when total kinetic energy drops below a threshold.
- **Node drag** — grab any node and reposition it; the simulation incorporates the new position in real time.
- **Auto-fit** — press `a` to frame all nodes in the viewport with configurable padding.

**Link Parsing**

- **Wikilinks** — extracts `[[Title]]` and `[[Title|Display Text]]` patterns from file content.
- **Frontmatter** — reads YAML `title:` and `tags: []` from `---`-delimited blocks.
- **Title resolution** — fallback chain: frontmatter `title:` → first `# heading` → filename stem. Links are resolved by case-insensitive matching against titles.

**Visualization**

- **4 node color modes** — `tag` (golden-ratio HSL per tag for maximum distinction), `folder` (color by parent directory), `link_count` (interpolated palette based on connection count), `uniform`.
- **3 edge color modes** — `source` (inherit source node color), `target` (inherit target node color), `uniform`.
- **Multi-tag indicators** — the primary (first) tag determines the node color; additional tags appear as small colored dots orbiting the node.
- **3 node shapes** — circle, square, diamond (drawn as outlined polygons on the canvas).
- **4 label modes** — `selected` (selected node only), `neighbors` (selected + connected), `all`, `none`.
- **3 canvas markers** — braille (high detail), halfblock, dot.

**Navigation**

- **Minimap** — bird's-eye overview with a viewport rectangle; click or drag inside to navigate. Configurable position (4 corners) and size.
- **Keyboard navigation** — arrow keys select the nearest node in that direction using an angle-weighted scoring function.
- **Viewport** — smooth pan, zoom (keyboard + scroll), and auto-fit with configurable sensitivity and padding.

**Search**

- Press `f` to open a popup that fuzzy-matches against node titles, relative file paths, and tags (case-insensitive). Navigate results with arrows or Tab, press Enter to jump.

**Configuration**

- **3-layer override** — defaults → TOML config file (`~/.config/graf/config.toml`) → CLI arguments → `GRAF_*` environment variables.
- **11 built-in themes** — default (terminal-native), Tokyo Night, Catppuccin Mocha, One Dark, Gruvbox, Dracula, Nord, Rose Pine, Everforest, Kanagawa, Solarized.
- **Hex color overrides** — per-element (`node_color`, `edge_color`, `label_color`, etc.) on top of any theme.
- **Background modes** — `transparent` (passes through terminal transparency) or `solid` (theme background color).

**Overlays**

- **Status bar** — template-based with variables: `{files}`, `{links}`, `{selected}`, `{date}`, `{time}`, `{size}`, `{ratio}`.
- **Legend** — shows tag or folder color mapping; configurable position and max items.
- **Grid overlay** — configurable number of divisions per axis.
- **Help overlay** — press `?` for a quick reference of all controls.

**Integration**

- **Editor** — opens files in `$EDITOR` (or configured command) with terminal suspend/resume. The RAII `TerminalGuard` ensures the terminal is always restored, even on panic.
- **Filtering** — exclude files by tags, glob patterns, minimum link count, or cap total nodes.
- **Config validation** — invalid values show a popup with valid suggestions on startup.
- **Live refresh** — press `r` to rescan files and restart the simulation without leaving the app.

## Motivation

[Obsidian](https://obsidian.md)'s graph view inspired this project. `graf` brings similar interactive graph visualization to the terminal — keyboard-first, runs over SSH, integrates with any `$EDITOR`, zero GUI overhead, and works inside tmux.

## Installation

### From source

```bash
git clone <repo-url>
cd graf
cargo install --path .
```

### Dependencies

- Rust 1.85+ (Edition 2024)

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
| `f` | Activate search |
| `r` | Refresh file scan |
| `Shift+M` | Toggle minimap |
| `Shift+L` | Toggle legend |
| `Shift+G` | Toggle grid |
| `Shift+S` | Toggle status bar |
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

## Search

Press `f` to activate the search popup. Search matches against node titles, file paths (relative), and tags (case-insensitive).

### Search controls

| Key | Action |
|-----|--------|
| `Enter` | Select result and jump to node |
| `↑` `↓` | Navigate results |
| `Tab` / `Shift+Tab` | Cycle forward/backward |
| `Esc` | Close search |
| `Ctrl+A` / `Ctrl+E` | Move to start/end of query |
| `Ctrl+U` | Clear entire query |
| `Ctrl+W` | Delete word backward |

## Comparison with Obsidian's Graph View

Obsidian's [graph view](https://help.obsidian.md/Plugins/Graph+view) was the direct inspiration for this project. Both share core capabilities: force-directed layout, node drag, pan/zoom, wikilink parsing, frontmatter tags, and search. The table below highlights where they differ:

| | Obsidian Graph View | graf |
|---|---|---|
| Platform | GUI (Electron desktop app) | Terminal (any emulator, SSH, tmux) |
| Input | Mouse | Mouse + full keyboard navigation |
| Node coloring | Tag (via community plugins) | Tag, folder, link count, uniform (built-in) |
| Node shapes | Circle only | Circle, square, diamond |
| Minimap | No | Yes (interactive, 4 positions) |
| Label modes | Hover to show | Selected, neighbors, all, none |
| Config | GUI settings | TOML file + CLI flags + env vars |
| Editor | Built-in Obsidian editor | Any `$EDITOR` (vim, nvim, helix, etc.) |
| Live file watching | Yes | Manual refresh (`r`) |

## What graf is not

- **Not a markdown editor** — `graf` visualizes links between files. To edit, it opens files in your `$EDITOR`.
- **Not a note-taking app** — it reads existing markdown directories. There is no vault, no database, no indexing service.
- **No live filesystem watching** — files are scanned once on launch. Press `r` to rescan.
- **No embedded note preview** — use your editor for reading and writing.
- **No collaboration or sync** — single-user, single-session terminal tool.
- **Not a replacement for Obsidian** — it complements terminal-centric workflows where a GUI is unavailable or unnecessary.

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
show_minimap = true
minimap_position = "bottom_right"
minimap_width = 30
minimap_height = 12
canvas_marker = "braille"
node_shape = "circle"
label_offset = 4.0
grid_divisions = 10

[visual.colors]
node_color = "#ff6600"
edge_color = "#333333"
label_color = "#aaaaaa"
selection_ring_color = "#ffffff"
border_color = "#555555"
grid_color = "#222222"

[physics]
ideal_distance = 80.0
damping = 0.95
max_iterations = 800
gravity = 0.01
cooling = true
prevent_overlapping = true
timestep = 0.016
thread_sleep_ms = 16

[interaction]
double_click_ms = 300
zoom_factor = 1.15
drag_sensitivity = 0.5
auto_fit_padding = 1.4
drag_scale = 200.0

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

[search]
max_results = 20
max_visible = 10
popup_width = 50
popup_y = 3
cursor_glyph = "▎"

[editor]
command = ""
```

### `[visual]`

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `theme` | string | `"default"` | Color scheme preset (see **Themes** below) |
| `background` | string | `"transparent"` | `"transparent"` passes through terminal transparency, `"solid"` fills with theme background color |
| `node_color_mode` | string | `"tag"` | How nodes are colored: `"tag"`, `"folder"`, `"link_count"`, `"uniform"` |
| `edge_color_mode` | string | `"source"` | Edge color: `"source"`, `"target"`, `"uniform"` |
| `label_mode` | string | `"selected"` | Which labels to show: `"selected"`, `"neighbors"`, `"all"`, `"none"` |
| `label_max_length` | int | `20` | Max characters for title labels (1–60) |
| `node_size` | float | `2.0` | Base node radius (1–5) |
| `node_size_mode` | string | `"fixed"` | `"fixed"` constant size, `"link_count"` scales with connections |
| `edge_thickness` | int | `1` | Line thickness for edges (1–3) |
| `show_legend` | bool | `true` | Show tag legend |
| `show_grid` | bool | `false` | Show background grid overlay |
| `show_minimap` | bool | `true` | Show minimap in corner |
| `minimap_position` | string | `"top_right"` | Minimap position: `"top_right"`, `"top_left"`, `"bottom_right"`, `"bottom_left"` |
| `minimap_width` | int | `30` | Minimap width in columns |
| `minimap_height` | int | `12` | Minimap height in rows |
| `canvas_marker` | string | `"braille"` | Canvas rendering marker: `"braille"`, `"halfblock"`, `"dot"` |
| `minimap_marker` | string | `"braille"` | Minimap rendering marker: `"braille"`, `"halfblock"`, `"dot"` |
| `node_shape` | string | `"circle"` | Node shape: `"circle"`, `"square"`, `"diamond"` |
| `label_offset` | float | `4.0` | Distance between node edge and label |
| `grid_divisions` | int | `10` | Number of grid lines per axis (2–50) |

### `[visual.colors]`

Optional hex color overrides applied on top of the selected theme. All fields are optional.

| Key | Type | Description |
|-----|------|-------------|
| `node_color` | hex string | Override all node colors (e.g. `"#ff6600"`) |
| `edge_color` | hex string | Override edge color |
| `label_color` | hex string | Override label text color |
| `selection_ring_color` | hex string | Override selection ring color |
| `border_color` | hex string | Override border, legend border, and minimap border |
| `title_color` | hex string | Override title color |
| `grid_color` | hex string | Override grid line color |
| `legend_text_color` | hex string | Override legend text color |
| `status_bar_color` | hex string | Override status bar text color |
| `background_color` | hex string | Override canvas and minimap background color |

### `[physics]`

Controls the force-directed layout simulation.

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `ideal_distance` | float | `80.0` | Target distance between connected nodes |
| `damping` | float | `0.95` | Velocity damping per step (lower = faster settling) |
| `max_iterations` | int | `800` | Maximum simulation steps before stopping |
| `gravity` | float | `0.01` | Center pull to prevent drift |
| `cooling` | bool | `true` | Whether simulation cools down over time |
| `prevent_overlapping` | bool | `true` | Prevent nodes from overlapping |
| `timestep` | float | `0.016` | Simulation timestep (lower = smoother but slower) |
| `thread_sleep_ms` | int | `16` | Milliseconds between simulation steps (~60fps) |

### `[interaction]`

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `double_click_ms` | int | `300` | Time window in milliseconds to register a double-click |
| `zoom_factor` | float | `1.15` | Zoom multiplier per scroll or key press |
| `drag_sensitivity` | float | `0.5` | Mouse drag speed multiplier |
| `auto_fit_padding` | float | `1.4` | Padding multiplier when auto-fitting view (higher = more zoomed out) |
| `drag_scale` | float | `200.0` | Internal scaling factor for background drag panning |

### `[display]`

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `show_status_bar` | bool | `true` | Show the status bar at the bottom |
| `status_format` | string | `"Files: {files} \| Links: {links} \| Selected: {selected}"` | Status bar text template. Variables: `{files}`, `{links}`, `{selected}`, `{date}`, `{time}`, `{size}`, `{ratio}` |
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

### `[search]`

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `max_results` | int | `20` | Maximum search results to return |
| `max_visible` | int | `10` | Maximum results visible without scrolling |
| `popup_width` | int | `50` | Search popup width in columns |
| `popup_y` | int | `3` | Vertical position of search popup from top |
| `cursor_glyph` | string | `"▎"` | Cursor character in search input |

### `[editor]`

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `command` | string | `""` | Editor command. Falls back to `$EDITOR` env var, then `vim` |

## CLI Arguments

```
graf [OPTIONS]

Options:
  -d, --dir <DIR>              Directory to scan [default: current]
  -c, --config <CONFIG>        Path to config file
      --theme <THEME>          Theme preset
      --max-nodes <MAX_NODES>  Maximum nodes to display
      --exclude <EXCLUDE>      Exclude glob patterns (repeatable)
      --exclude-tags <TAGS>    Exclude tags (comma-separated)
      --node-color-mode <MODE> Node color mode
      --edge-color-mode <MODE> Edge color mode
      --label-mode <MODE>      Label display mode
      --labels                 Show all labels (shorthand for --label-mode all)
      --no-status              Hide status bar
      --grid                   Show grid
      --no-minimap             Hide minimap
      --no-legend              Hide legend
      --background <BG>        Background style
      --border-style <STYLE>   Border style for overlays
      --editor <EDITOR>        Editor command to open files
  -h, --help                   Print help
  -V, --version                Print version
```

CLI arguments override config file values, which override defaults.

## Environment variables

Config values can be overridden with `GRAF_*` environment variables (uppercase, underscores):

```bash
GRAF_VISUAL_THEME="dracula" GRAF_FILTER_MAX_NODES=200 graf
```

Common variables:
- `GRAF_VISUAL_THEME` — Theme preset
- `GRAF_EDITOR_COMMAND` — Editor command
- `GRAF_FILTER_MAX_NODES` — Maximum nodes
- `GRAF_FILTER_MIN_LINKS` — Minimum links threshold
- `GRAF_VISUAL_NODE_COLOR_MODE` — Node color mode
- `GRAF_DISPLAY_SHOW_STATUS_BAR` — Show status bar (true/false)

Format: `GRAF_{SECTION}_{KEY}` in all caps with underscores.

## Themes

### Presets

| Value | Description |
|-------|-------------|
| `"default"` | Inherits colors from your terminal's color scheme. No hardcoded palette. |
| `"tokyo-night"` | Tokyo Night (dark) palette |
| `"catppuccin-mocha"` | Catppuccin Mocha palette |
| `"onedark"` | One Dark palette |
| `"gruvbox"` | Gruvbox Dark palette |
| `"dracula"` | Dracula palette |
| `"nord"` | Nord Frost palette |
| `"rose-pine"` | Rose Pine palette |
| `"everforest"` | Everforest palette |
| `"kanagawa"` | Kanagawa palette |
| `"solarized"` | Solarized Dark palette |

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
| `clap` | CLI argument parsing |
| `glob` | File discovery |
| `chrono` | Date/time formatting for status bar |
| `directories` | XDG config path resolution |

## Project structure

```
graf/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs          # Entry point, terminal setup, event loop
    ├── cli.rs           # CLI argument definitions (clap)
    ├── app.rs           # App state management, graph initialization
    ├── config.rs        # Config loading, validation, color overrides
    ├── config/
    │   └── themes.rs    # 11 theme palettes and builder
    ├── linker.rs        # File scanning, wikilink extraction, frontmatter parsing
    ├── ui.rs            # Help overlay, search popup, config error rendering
    ├── util.rs          # Shared utilities (text truncation)
    └── graph/
        ├── mod.rs       # GraphState, ForceGraph construction, search
        ├── render.rs    # Canvas drawing (nodes, edges, labels, legend, grid, minimap)
        ├── input.rs     # Keyboard and mouse event handling
        ├── viewport.rs  # Pan, zoom, screen-to-world conversion, hit-testing
        └── physics.rs   # Background thread for force simulation
```

## Performance

- Simulation runs at ~60fps (default `thread_sleep_ms = 16`)
- Simulation auto-stops when total energy drops below `0.05 * node_count`
- Files are sorted alphabetically by path before graph construction
- Max zoom: ~20x initial; Min zoom: 0.01 (hardcoded floor)

## Edge cases

- **Empty directory**: Exits with error "No markdown files found"
- **Self-links**: Filtered out automatically
- **Unresolved wikilinks**: Silently ignored
- **Title fallback**: Frontmatter → first `# heading` → filename stem
- **Nodes without tags**: Default to `Color::Gray` in tag mode
- **Config errors**: Invalid values show popup with valid suggestions
