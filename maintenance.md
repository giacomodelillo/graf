# FINAL IMPLEMENTATION PLAN

## Overview

| Phase | Scope | Files Changed | Est. Lines Removed | Est. Lines Added | Risk |
|-------|-------|--------------|-------------------|-----------------|------|
| 0 | Theme & config architecture | config.rs, new themes.rs, new default_config.toml | ~380 | ~200 | Low |
| 1 | Clippy auto-fix + dead code | 8 files | ~80 | ~40 | Lowest |
| 2 | Refactoring (DRY, KISS) | main.rs, input.rs, render.rs, ui.rs, app.rs | ~120 | ~60 | Low |
| 3 | Bug fixes | viewport.rs, main.rs, linker.rs, ui.rs | ~10 | ~30 | Medium |
| 4 | Performance caching | app.rs, config.rs, render.rs, linker.rs | ~15 | ~50 | Medium |
| 5 | Security hardening | main.rs, config.rs | ~5 | ~20 | Low |

**Net result**: ~4105 → ~3600 lines, 28 clippy warnings → 0, 0 new deps, 1 dep removed (`once_cell`).

---

## PHASE 0: Theme & Config Architecture

### 0A. Create `src/default_config.toml`

**New file**: Extract the raw TOML string from `src/config.rs:1326-1443` into a standalone file.

- Move `r###"..."###` content (118 lines) verbatim to `src/default_config.toml`
- Strip the `r###"` and `"###.to_string()` wrapper
- Fix minor inconsistency: default TOML says `show_minimap = false` but `VisualConfig::default()` has `show_minimap: false` — **these match**, good. But `show_legend = true` in TOML vs `VisualConfig::default()` has `show_legend: default_true()` — **these match**, good.
- Fix: TOML says `node_color_mode = "folder"` but `VisualConfig::default()` has `node_color_mode: NodeColorMode::Folder` — matches.
- Fix: TOML says `edge_color_mode = "uniform"` but default has `EdgeColorMode::Uniform` — matches.
- Note: TOML has `legend.max_items = 100` but `default_max_legend_items()` returns `10`. After this phase, fix default TOML to match code default (`10`), and note this as a discrepancy fixed.

### 0B. Create `src/themes.rs`

**New file**: ~90 lines. Defines compact theme palettes and builder.

```rust
use ratatui::style::Color;

use super::config::{Background, Theme, ThemeColors};

struct ThemePalette {
    nodes: [[u8; 3]; 8],
    chrome: [u8; 3],
    title: [u8; 3],
    text: [u8; 3],
    fg: [u8; 3],
    grid: [u8; 3],
    bg: [u8; 3],
}

impl ThemePalette {
    const fn rgb(c: [u8; 3]) -> Color {
        Color::Rgb(c[0], c[1], c[2])
    }

    fn build(&self, background: Background) -> ThemeColors {
        ThemeColors {
            node_colors: self.nodes.map(Self::rgb).to_vec(),
            edge_color: Self::rgb(self.chrome),
            border_color: Self::rgb(self.chrome),
            title_color: Self::rgb(self.title),
            label_color: Self::rgb(self.text),
            legend_text_color: Self::rgb(self.text),
            legend_border_color: Self::rgb(self.chrome),
            selected_indicator_color: Self::rgb(self.fg),
            grid_color: Self::rgb(self.grid),
            background_color: match background {
                Background::Transparent => None,
                Background::Solid => Some(Self::rgb(self.bg)),
            },
            status_bar_color: Self::rgb(self.chrome),
            minimap_border_color: Self::rgb(self.chrome),
            minimap_viewport_color: Self::rgb(self.fg),
            minimap_bg_color: Some(Self::rgb(self.bg)),
        }
    }
}
```

Then 10 const `ThemePalette` definitions — one per RGB theme. Each is 12 lines (7 fields + padding). Total: ~120 lines.

Then a lookup function:

```rust
pub fn theme_colors(theme: Theme, background: Background) -> ThemeColors {
    match theme {
        Theme::Default => default_theme_colors(background),
        Theme::TokyoNight => PALETTES[0].build(background),
        Theme::CatppuccinMocha => PALETTES[1].build(background),
        Theme::Onedark => PALETTES[2].build(background),
        Theme::Gruvbox => PALETTES[3].build(background),
        Theme::Dracula => PALETTES[4].build(background),
        Theme::Nord => PALETTES[5].build(background),
        Theme::RosePine => PALETTES[6].build(background),
        Theme::Everforest => PALETTES[7].build(background),
        Theme::Kanagawa => PALETTES[8].build(background),
        Theme::Solarized => PALETTES[9].build(background),
    }
}
```

`default_theme_colors()` — special case, ~25 lines, uses named `Color` variants (DarkGray, Gray, Reset, White, etc.) as current code does. Cannot use RGB palette because it relies on terminal's color scheme.

### 0C. Modify `src/config.rs`

1. **Add `mod themes;`** — `themes.rs` is in `src/`, and it depends on `config::ThemeColors`, `config::Background`, and `config::Theme`. Use Option A: `themes` is a submodule of `config` (`config/themes.rs`), `mod themes;` inside `config.rs`. This keeps theme logic co-located with config types.

2. **Replace `theme_colors()` method** on `GrafConfig` (lines 770-1117, ~348 lines) with:

```rust
pub fn theme_colors(&self) -> ThemeColors {
    let mut colors = themes::theme_colors(self.visual.theme.clone(), self.visual.background);
    // apply color overrides (existing logic, ~30 lines, stays here)
    ...
    colors
}
```

The override block (lines 1082-1114, ~33 lines) stays in `config.rs` — it's config-specific logic.

3. **Replace `generate_default_toml()`** (lines 1325-1443, ~119 lines) with:

```rust
fn generate_default_toml() -> &'static str {
    include_str!("default_config.toml")
}
```

Signature changes from `-> String` to `-> &'static str`. Caller at line 1207 passes result to `fs::write()` — works fine, `fs::write` accepts `AsRef<[u8]>`.

4. **Net change to config.rs**: Remove ~460 lines (348 theme match arms + 119 raw string), add ~35 lines (delegation + `include_str`). config.rs goes from 1404 → ~980 lines.

### 0D. Fix discrepancy: `LegendConfig::max_items` default

In `config.rs` line 582-586:
```rust
impl Default for LegendConfig {
    fn default() -> Self {
        Self {
            position: LegendPosition::BottomRight,
            max_items: 100,  // ← BUG: should be 10
        }
    }
}
```

`default_max_legend_items()` returns `10`. The `Default` impl returns `100`. The generated TOML says `100`.

**Decision**: Change `Default` impl to `10` and update `default_config.toml` to `max_items = 10`. The `default_max_legend_items` function is the canonical source.

### 0E. Verification

- `cargo check` — must pass
- `cargo clippy` — theme code has no new warnings
- `default_config.toml` valid TOML (verified by syntax)

---

## PHASE 1: Clippy Auto-Fixes + Dead Code

### 1A. `cargo clippy --fix --bin "graf" -p graf --allow-dirty`

Auto-applies 24 of 28 warnings:
- 12x `collapsible_if` — collapse nested `if let Some(x) { if let Ok(y) { ... }}` → `if let Some(x) && let Ok(y) { ... }`
- 2x `derivable_impls` — `BorderStyle::default()`, `LegendPosition::default()` → `#[derive(Default)]` with `#[default]` attr
- 1x `let_and_return` — `expand_status` final binding
- 1x `unnecessary_map_or` — `minimap_area.map_or(false, ...)`
- 1x `unnecessary_filter_map` — render.rs minimap nodes
- 1x `approx_constant` — `1.0472` → `std::f64::consts::FRAC_PI_3`
- 2x `needless_borrow` — `&guard` → `guard` in input.rs
- 1x `if_same_then_else` — linker.rs dead branch

Files affected: `main.rs`, `config.rs`, `input.rs`, `linker.rs`, `graph/mod.rs`, `graph/viewport.rs`, `graph/render.rs`, `graph/physics.rs`

### 1B. Manual dead code removal

**Remove `src/util.rs`** — file exists but is never imported (`mod util` absent from all files). Delete it.

**Remove dead branch in `linker.rs:32-36`** (auto-fixed by clippy `if_same_then_else`):
```rust
// BEFORE (clippy already auto-fixed this, but verify)
let patterns = if exclude_patterns.is_empty() {
    vec!["**/*.md".to_string(), "**/*.mdx".to_string()]
} else {
    vec!["**/*.md".to_string(), "**/*.mdx".to_string()]
};

// AFTER
let patterns = ["**/*.md", "**/*.mdx"];
// Then change usage from `for pattern in &patterns` to iterate this array
```

**Remove unused `_total_edges` return from `build_graph`** — `graph/mod.rs:68,97`:
```rust
// BEFORE
pub fn build_graph(files, config) -> (ForceGraph<GraphNodeData, ()>, usize) {
    ...
    let total_edges: usize = links.values().map(|v| v.len()).sum(); // line 68 — dead
    ...
    (graph, total_edges) // line 97
}

// AFTER
pub fn build_graph(files: &[FileData], config: &GrafConfig) -> ForceGraph<GraphNodeData, ()> {
    ...
    graph
}
```

Update callers:
- `src/app.rs:30`: `let (graph, _total_edges) = crate::graph::build_graph(...)` → `let graph = crate::graph::build_graph(...)`
- `src/app.rs:73`: same change

**Remove duplicate help text** — `src/ui.rs:125`:
```rust
// Line 125 is duplicate "  f           Search nodes" — delete it
```

**Replace `once_cell::sync::Lazy` with `std::sync::LazyLock`** — `src/linker.rs:6,9-15`:
```rust
// BEFORE
use once_cell::sync::Lazy;
static WIKI_LINK_RE: Lazy<Regex> = Lazy::new(|| ...);

// AFTER
use std::sync::LazyLock;
static WIKI_LINK_RE: LazyLock<Regex> = LazyLock::new(|| ...);
```

Apply to all 4 statics: `WIKI_LINK_RE`, `FRONTMATTER_RE`, `TAGS_RE`, `TITLE_RE`.

**Remove `once_cell` from `Cargo.toml`** — line 13:
```toml
# DELETE this line:
once_cell = "1"
```

### 1C. Consolidate `truncate` functions

**Keep `util::truncate`** — wait, `util.rs` is being deleted. Instead, pick one location.

Best location: `src/util.rs` is dead. Create a proper `src/util.rs` that's actually used:

Actually, simplest: put it in a shared location. Since `render.rs` and `ui.rs` both need it, and both are in `crate::`, create `src/util.rs` properly:

1. Re-create `src/util.rs` with only `truncate()`
2. Add `mod util;` to `src/main.rs` (currently missing — that's why it was dead)
3. Replace `truncate_owned` in `render.rs:707` and `truncate_display` in `ui.rs:250` with calls to `crate::util::truncate`

Wait — `util.rs` has `mod` declared nowhere. Let me check `main.rs`:

Looking at `main.rs:1-6`:
```rust
mod app;
mod cli;
mod config;
mod graph;
mod linker;
mod ui;
```

No `mod util;`. The file exists but is unreachable. Two options:
- **Option A**: Add `mod util;` to main.rs, consolidate truncate there
- **Option B**: Put truncate inline where most used (render.rs) and import from there

**Choose Option A**: Add `mod util;` to main.rs. util.rs keeps `truncate()`. Remove `truncate_owned` and `truncate_display`. Call `crate::util::truncate()` from both sites.

### 1D. Verification

- `cargo check`
- `cargo clippy -- -W clippy::all` — target: ≤5 warnings (down from 28)
- Confirm `once_cell` not in `Cargo.lock` dependency tree

---

## PHASE 2: Refactoring (DRY, KISS)

### 2A. Extract `GraphAction` enum — `src/graph/input.rs`

Replace `Option<String>` return type with typed enum:

```rust
pub enum GraphAction {
    Quit,
    OpenFile(String),
    ToggleHelp,
    ToggleSearch,
    ToggleMinimap,
    ToggleLegend,
    ToggleGrid,
    ToggleStatus,
    Refresh,
}

pub fn handle_graph_keys(...) -> Option<GraphAction> { ... }
pub fn handle_graph_mouse(...) -> Option<GraphAction> { ... }
```

Update callers in `src/main.rs` — the match on `action.as_str()` (lines 111-146, 162-165) becomes cleaner, no string parsing.

### 2B. Extract `dispatch_action` — `src/main.rs`

DRY the duplicated event-action match in main loop (lines 282-311):

```rust
fn dispatch_action(
    action: EventAction,
    app_state: &mut AppState,
    guard: &mut TerminalGuard,
    config: &GrafConfig,
    running: &mut bool,
) -> Result<()> {
    match action {
        EventAction::Quit => {
            app_state.shutdown();
            *running = false;
        }
        EventAction::OpenFile(path) => {
            guard.suspend()?;
            open_file_in_editor(&path, config);
            guard.resume()?;
        }
    }
    Ok(())
}
```

Then both outer and inner loop just call `dispatch_action(action, &mut app_state, &mut guard, config, &mut running)?`.

### 2C. Extract `GraphState::new()` — `src/app.rs`

Both `AppState::new()` (lines 30-46) and `refresh_simulation()` (lines 73-87) construct `GraphState` identically:

```rust
impl GraphState {
    pub fn new(files: &[FileData], config: &GrafConfig) -> Self {
        let graph = crate::graph::build_graph(files, config);
        let simulation = crate::graph::create_simulation(graph, config);
        let mut state = GraphState {
            viewport: viewport::Viewport::default(),
            simulation,
            selected_node: None,
            dragging_node: None,
            drag_target: None,
            is_settled: false,
        };
        state.viewport = state.viewport.auto_fit_from_graph(
            state.simulation.get_graph(),
            config.interaction.auto_fit_padding,
        );
        state
    }
}
```

Then `AppState::new()` and `refresh_simulation()` both call `GraphState::new()`.

### 2D. Extract `BorderStyle::to_border_type()` — `src/config.rs`

Currently duplicated in `ui.rs:7-14` and `render.rs:503-508`:

```rust
impl BorderStyle {
    pub fn to_border_type(&self) -> ratatui::widgets::BorderType {
        match self {
            BorderStyle::Plain => ratatui::widgets::BorderType::Plain,
            BorderStyle::Rounded => ratatui::widgets::BorderType::Rounded,
            BorderStyle::Double => ratatui::widgets::BorderType::Double,
            BorderStyle::None => ratatui::widgets::BorderType::Plain,
        }
    }
}
```

Replace both call sites:
- `ui.rs`: `border_type_from_config(config)` → `config.display.border_style.to_border_type()`
- `render.rs`: inline match → `config.display.border_style.to_border_type()`
- Delete `border_type_from_config` function

### 2E. Extract `draw_shape` helper — `src/graph/render.rs`

Node shape drawing (circle/square/diamond) is duplicated for:
1. Main node body (lines 85-169)
2. Selection ring (lines 196-280)

Extract:

```rust
fn draw_outlined_shape(
    painter: &mut Painter,
    cx: f64, cy: f64, radius: f64,
    shape: NodeShape, color: Color,
) {
    match shape {
        NodeShape::Circle => { /* 16-segment circle */ }
        NodeShape::Square => { /* 4 lines */ }
        NodeShape::Diamond => { /* 4 lines */ }
    }
}
```

Then `GraphNodesShape::draw` calls `draw_outlined_shape` twice per node (once for body, once for ring if selected). Reduces ~200 lines to ~60.

### 2F. Verification

- `cargo check`
- `cargo clippy -- -W clippy::all` — target: 0 warnings

---

## PHASE 3: Bug Fixes

### 3A. Fix `unwrap()` on `graph_state` in search — `src/main.rs`

**`handle_search_keys` Enter handler (lines 359-386)**:
```rust
// BEFORE
let guard = app_state.graph_state.as_ref().unwrap().read()...

// AFTER
let Some(graph_state) = &app_state.graph_state else { return; };
let guard = graph_state.read().unwrap_or_else(|e| e.into_inner());
```

Same fix at `run_search` (line 509):
```rust
let Some(graph_state) = &app_state.graph_state else { return; };
```

### 3B. Fix `Viewport::x_bounds` / `y_bounds` to use `aspect` — `src/graph/viewport.rs`

Currently ignores `aspect` param. The canvas coordinate system needs to account for terminal aspect ratio.

**Pragmatic fix**: Keep viewport logic unchanged. Add a code comment documenting the dot marker limitation.

**Revised decision on BUG-11**: Defer viewport aspect fix. Current behavior is correct for braille and halfblock (99% of users). Add a code comment documenting the dot marker limitation.

### 3C. Fix `truncate` byte-boundary edge case — `src/util.rs`

Current implementation:
```rust
pub fn truncate(s: &str, max_len: usize) -> String {
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
```

Fix: add the `end == 0` guard for multi-byte first char edge case:

```rust
pub fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        return s.to_string();
    }
    if max_len == 0 {
        return String::new();
    }
    let mut end = max_len;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    if end == 0 {
        return String::new();
    }
    let mut end = end.saturating_sub(1);
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    if end == 0 {
        return s.chars().next().map(|c| format!("{}…", c)).unwrap_or_default();
    }
    format!("{}…", &s[..end])
}
```

### 3D. Add search query length cap — `src/main.rs`

In `handle_search_keys`, before `search_query.insert`:

```rust
KeyCode::Char(c) if !ctrl => {
    const MAX_SEARCH_LEN: usize = 256;
    if app_state.search_query.len() < MAX_SEARCH_LEN {
        app_state.search_query.insert(app_state.search_cursor, c);
        app_state.search_cursor += c.len_utf8();
        run_search(app_state, config);
    }
}
```

### 3E. Verification

- `cargo check`
- `cargo clippy`

---

## PHASE 4: Performance Caching

### 4A. Cache `theme_colors` — `src/config.rs`

**Decision**: Skip theme_colors caching. The computation is ~5μs (10 theme lookups + 10 override checks), and it's called 2x per frame (render + ui), total ~10μs/frame at 60fps = 0.06% CPU. Not worth caching.

### 4B. Cache `graph_bounds` — `src/graph/render.rs`

`compute_graph_bounds` is called 3 times per frame (status bar, minimap, minimap_screen_to_world in input). Cache in `GraphState`:

```rust
pub struct GraphState {
    pub simulation: Simulation<GraphNodeData, ()>,
    pub viewport: viewport::Viewport,
    pub selected_node: Option<NodeIndex>,
    pub dragging_node: Option<NodeIndex>,
    pub drag_target: Option<(f32, f32)>,
    pub is_settled: bool,
    pub graph_bounds: (f64, f64, f64, f64), // (min_x, max_x, min_y, max_y)
}
```

Compute once in `GraphState::new()` and update in physics thread after simulation step. Or simpler: compute once in `draw_graph_view` and pass to all consumers.

**Simpler approach**: Compute once in `draw_graph_view`, pass to status bar and minimap rendering:

```rust
let bounds = compute_graph_bounds(graph);
// ... use bounds for status bar calculation ...
// ... pass bounds to draw_minimap ...
```

Currently `draw_minimap` calls `compute_graph_bounds` internally. Change signature to accept bounds.

Similarly in `input.rs:minimap_screen_to_world`, accept bounds parameter.

### 4C. Pre-resolve exclude patterns — `src/linker.rs`

Current: `should_exclude` runs `glob::glob()` per file per pattern. O(files * patterns * FS).

Fix: Pre-resolve patterns once in `scan_markdown_files`:

```rust
fn resolve_exclude_set(base_dir: &Path, patterns: &[String]) -> HashSet<PathBuf> {
    let mut excluded = HashSet::new();
    for pat in patterns {
        if let Ok(paths) = glob(&base_dir.join(pat).to_string_lossy()) {
            for path in paths.flatten() {
                excluded.insert(path);
            }
        }
    }
    excluded
}
```

Then `should_exclude` becomes a simple `HashSet::contains` check. For substring patterns, keep those as a secondary check.

### 4D. Use `HashSet` for link dedup — `src/linker.rs`

In `resolve_links` (line 164-170):

```rust
// BEFORE
let mut targets = Vec::new();
for link in &file.wikilinks {
    if let Some(target) = title_to_path.get(&link.to_lowercase()) {
        if target != &file.relative_path && !targets.contains(target) {
            targets.push(target.clone());
        }
    }
}

// AFTER
let mut seen = HashSet::new();
let mut targets = Vec::new();
for link in &file.wikilinks {
    if let Some(target) = title_to_path.get(&link.to_lowercase()) {
        if target != &file.relative_path && seen.insert(target.clone()) {
            targets.push(target.clone());
        }
    }
}
```

Or even simpler: collect to `HashSet` then convert to `Vec`.

### 4E. Verification

- `cargo check`
- `cargo clippy`

---

## PHASE 5: Security Hardening

### 5A. Validate file paths — `src/main.rs:open_file_in_editor`

```rust
fn open_file_in_editor(relative_path: &str, config: &GrafConfig) {
    let cwd = std::env::current_dir().unwrap_or_default();
    let full_path = cwd.join(relative_path);
    
    // Canonicalize to resolve symlinks and ..
    let full_path = match full_path.canonicalize() {
        Ok(p) => p,
        Err(_) => return,
    };
    
    // Ensure path is under cwd (prevent path traversal)
    if !full_path.starts_with(&cwd) {
        return;
    }
    
    let editor = if !config.editor.command.is_empty() {
        config.editor.command.clone()
    } else {
        std::env::var("EDITOR").unwrap_or_else(|_| "vim".to_string())
    };

    if let Err(e) = std::process::Command::new(&editor).arg(&full_path).status() {
        eprintln!("Failed to open editor '{}': {}", editor, e);
    }
}
```

### 5B. Set config file permissions — `src/config.rs:load_from_path`

After writing config:

```rust
if let Some(parent) = path.parent() {
    let _ = fs::create_dir_all(parent);
    let _ = fs::write(&path, generate_default_toml());
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(&path, fs::Permissions::from_mode(0o600));
    }
    created = true;
}
```

### 5C. Log editor spawn errors — `src/main.rs:open_file_in_editor`

```rust
if let Err(e) = std::process::Command::new(&editor).arg(&full_path).status() {
    eprintln!("Failed to open editor '{}': {}", editor, e);
}
```

### 5D. Verification

- `cargo check` — ensure `#[cfg(unix)]` compiles correctly
- `cargo clippy`

---

## EXECUTION ORDER

```
Step 1:  Create src/config/themes.rs with ThemePalette struct + 10 palettes + builder
Step 2:  Create src/default_config.toml from raw string in config.rs
Step 3:  Edit config.rs — add mod themes, replace theme_colors(), replace generate_default_toml()
Step 4:  cargo check + cargo clippy (verify Phase 0)
Step 5:  cargo clippy --fix (auto-fix 24 warnings)
Step 6:  Manual fixes: once_cell → LazyLock, remove util.rs dead file, add mod util + truncate, remove dead branch, remove _total_edges, remove duplicate help text, fix LegendConfig default
Step 7:  cargo check + cargo clippy (verify Phase 1)
Step 8:  Refactoring: GraphAction enum, dispatch_action, GraphState::new(), BorderStyle method, draw_shape
Step 9:  cargo check + cargo clippy (verify Phase 2)
Step 10: Bug fixes: search unwrap, truncate edge case, search query cap
Step 11: cargo check + cargo clippy (verify Phase 3)
Step 12: Perf: graph_bounds caching, glob pre-resolve, HashSet dedup
Step 13: cargo check + cargo clippy (verify Phase 4)
Step 14: Security: path validation, config permissions, editor error logging
Step 15: Final: cargo check, cargo clippy — target: 0 warnings
```

Each step is verified before proceeding to the next. If any `cargo check` fails, stop and fix before continuing.

---

## RISK MITIGATION

| Risk | Mitigation |
|------|------------|
| Phase 0: Theme palette index mismatch | Verify palette order matches `Theme` enum order. Build test: construct all themes, compare RGB values to original |
| Phase 0: `include_str!` path wrong | File must be relative to config.rs location → `include_str!("default_config.toml")` since both in `src/` root |
| Phase 1: `once_cell` → `std::sync::LazyLock` API change | `LazyLock::new(|| ...)` is identical to `Lazy::new(|| ...)` |
| Phase 2: `GraphAction` enum breaks callers | Change is type-safe — compiler catches all call sites |
| Phase 3: truncate fix changes behavior | Only affects edge case: multi-byte first char with max_len < 4 |
| Phase 4: `graph_bounds` caching stale data | Bounds don't change between frames (graph structure is immutable during render) |
| Phase 5: `canonicalize` fails for non-existent files | Guard with `match`, return early |
