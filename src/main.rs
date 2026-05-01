mod app;
mod config;
mod graph;
mod linker;
mod ui;

use std::io;

use anyhow::{Context, Result};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<()> {
    let config = config::GrafConfig::load();

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

    let mut terminal = setup_terminal()?;
    let mut app_state = app::AppState::new(&config, files);
    let mut running = true;

    while running {
        terminal.draw(|frame| {
            ui::draw_ui(frame, &app_state, &config);
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            match event::read()? {
                Event::Key(key) => {
                    if app_state.show_help {
                        if key.kind == event::KeyEventKind::Press
                            && (key.code == crossterm::event::KeyCode::Esc
                                || key.code == crossterm::event::KeyCode::Char('?'))
                        {
                            app_state.show_help = false;
                        }
                        continue;
                    }

                    if let Some(graph_state) = &app_state.graph_state {
                        if let Some(action) =
                            graph::input::handle_graph_keys(graph_state, key, &config)
                        {
                            match action.as_str() {
                                "quit" => running = false,
                                "help" => app_state.show_help = true,
                                _ if action.starts_with("open:") => {
                                    let path = &action[5..];
                                    open_file_in_editor(path);
                                }
                                _ => {}
                            }
                        }
                    }
                }
                Event::Mouse(mouse_event) => {
                    if app_state.show_help {
                        continue;
                    }
                    if let Some(graph_state) = &app_state.graph_state {
                        if let Some(action) = graph::input::handle_graph_mouse(
                            graph_state,
                            mouse_event,
                            frame_area(&terminal)?,
                            &mut app_state.graph_mouse_state,
                            &config,
                        ) {
                            match action.as_str() {
                                _ if action.starts_with("open:") => {
                                    let path = &action[5..];
                                    open_file_in_editor(path);
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

    restore_terminal(terminal)?;
    Ok(())
}

fn frame_area(terminal: &Terminal<CrosstermBackend<io::Stdout>>) -> Result<ratatui::layout::Rect> {
    let size = terminal.size().context("failed to get terminal size")?;
    Ok(ratatui::layout::Rect::new(0, 0, size.width, size.height))
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(mut terminal: Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn open_file_in_editor(relative_path: &str) {
    let cwd = std::env::current_dir().unwrap_or_default();
    let full_path = cwd.join(relative_path);
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());

    let _ = std::process::Command::new("sh")
        .arg("-c")
        .arg(format!(
            "{} {}",
            editor,
            shell_escape(&full_path.to_string_lossy())
        ))
        .status();
}

fn shell_escape(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}
