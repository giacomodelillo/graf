mod app;
mod config;
mod graph;
mod linker;
mod ui;

use std::io;
use std::io::Write;

use anyhow::{Context, Result};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

struct TerminalGuard {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl TerminalGuard {
    fn new() -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal })
    }

    fn as_mut(&mut self) -> &mut Terminal<CrosstermBackend<io::Stdout>> {
        &mut self.terminal
    }

    fn suspend(&mut self) -> Result<()> {
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.terminal.show_cursor()?;
        io::stdout().flush()?;
        Ok(())
    }

    fn resume(&mut self) -> Result<()> {
        enable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            EnterAlternateScreen,
            EnableMouseCapture
        )?;
        self.terminal.clear()?;
        Ok(())
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
        let _ = self.terminal.show_cursor();
        let _ = io::stdout().flush();
    }
}

fn main() -> Result<()> {
    let (config, mut config_errors) = config::GrafConfig::load();

    // Startup config validation check
    config_errors.extend(config.validate());

    let cwd = std::env::current_dir().context("failed to get current directory")?;
    let files = linker::scan_markdown_files(
        &cwd,
        &config.filter.exclude_patterns,
        config.filter.max_nodes,
    );

    if files.is_empty() {
        eprintln!("No markdown files found in {}", cwd.display());
        std::process::exit(1);
    }

    let mut guard = TerminalGuard::new()?;
    let mut app_state = app::AppState::new(&config, files, config_errors);
    let mut running = true;

    while running {
        guard.as_mut().draw(|frame| {
            ui::draw_ui(frame, &app_state, &config);
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            match event::read()? {
                Event::Key(key) => {
                    if !app_state.config_errors.is_empty() {
                        app_state.config_errors.clear();
                        continue;
                    }
                    if key.kind != event::KeyEventKind::Press {
                        continue;
                    }
                    if app_state.show_help {
                        if key.code == crossterm::event::KeyCode::Esc
                            || key.code == crossterm::event::KeyCode::Char('?')
                        {
                            app_state.show_help = false;
                        }
                        continue;
                    }

                    if app_state.search_active {
                        handle_search_keys(&mut app_state, key);
                        continue;
                    }

                    if let Some(graph_state) = &app_state.graph_state {
                        if let Some(action) =
                            graph::input::handle_graph_keys(graph_state, key, &config)
                        {
                            match action.as_str() {
                                "quit" => {
                                    app_state.shutdown();
                                    running = false;
                                }
                                "help" => app_state.show_help = true,
                                "search" => {
                                    app_state.search_active = true;
                                }
                                _ if action.starts_with("open:") => {
                                    let path = &action[5..];
                                    guard.suspend()?;
                                    open_file_in_editor(path);
                                    guard.resume()?;
                                }
                                _ => {}
                            }
                        }
                    }
                }
                Event::Mouse(mouse_event) => {
                    if app_state.show_help || app_state.search_active {
                        continue;
                    }
                    if let Some(graph_state) = &app_state.graph_state {
                        if let Some(action) = graph::input::handle_graph_mouse(
                            graph_state,
                            mouse_event,
                            frame_area(&guard)?,
                            &mut app_state.graph_mouse_state,
                            &config,
                        ) {
                            match action.as_str() {
                                _ if action.starts_with("open:") => {
                                    let path = &action[5..];
                                    guard.suspend()?;
                                    open_file_in_editor(path);
                                    guard.resume()?;
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn frame_area(guard: &TerminalGuard) -> Result<ratatui::layout::Rect> {
    let size = guard
        .terminal
        .size()
        .context("failed to get terminal size")?;
    Ok(ratatui::layout::Rect::new(0, 0, size.width, size.height))
}

fn open_file_in_editor(relative_path: &str) {
    let cwd = std::env::current_dir().unwrap_or_default();
    let full_path = cwd.join(relative_path);
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());

    let _ = std::process::Command::new(&editor).arg(&full_path).status();
}

fn handle_search_keys(app_state: &mut app::AppState, key: crossterm::event::KeyEvent) {
    use crossterm::event::KeyCode;

    match key.code {
        KeyCode::Esc => {
            app_state.search_active = false;
            app_state.search_query.clear();
            app_state.search_results.clear();
            app_state.search_selected = 0;
        }
        KeyCode::Enter => {
            if let Some(&(idx, _)) = app_state.search_results.get(app_state.search_selected) {
                let (nx, ny) = {
                    let guard = app_state
                        .graph_state
                        .as_ref()
                        .unwrap()
                        .read()
                        .unwrap_or_else(|e| e.into_inner());
                    let graph = guard.simulation.get_graph();
                    if let Some(node) = graph.node_weight(idx) {
                        (node.location.x, node.location.y)
                    } else {
                        return;
                    }
                };
                let mut guard = app_state
                    .graph_state
                    .as_ref()
                    .unwrap()
                    .write()
                    .unwrap_or_else(|e| e.into_inner());
                guard.selected_node = Some(idx);
                guard.viewport.center_on_node(nx, ny);
            }
            app_state.search_active = false;
            app_state.search_query.clear();
            app_state.search_results.clear();
            app_state.search_selected = 0;
        }
        KeyCode::Up => {
            if app_state.search_selected > 0 {
                app_state.search_selected -= 1;
            }
        }
        KeyCode::Down => {
            if !app_state.search_results.is_empty()
                && app_state.search_selected < app_state.search_results.len() - 1
            {
                app_state.search_selected += 1;
            }
        }
        KeyCode::Tab => {
            if !app_state.search_results.is_empty() {
                app_state.search_selected =
                    (app_state.search_selected + 1) % app_state.search_results.len();
            }
        }
        KeyCode::Backspace => {
            app_state.search_query.pop();
            run_search(app_state);
        }
        KeyCode::Char(c) => {
            app_state.search_query.push(c);
            run_search(app_state);
        }
        _ => {}
    }
}

fn run_search(app_state: &mut app::AppState) {
    if let Some(graph_state) = &app_state.graph_state {
        let guard = graph_state.read().unwrap_or_else(|e| e.into_inner());
        app_state.search_results = graph::search_nodes(&guard.simulation, &app_state.search_query);
    }
    app_state.search_selected = 0;
}
