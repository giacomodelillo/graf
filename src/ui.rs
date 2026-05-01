use ratatui::layout::Rect;
use ratatui::Frame;

use crate::app::AppState;
use crate::config::GrafConfig;

pub fn draw_ui(frame: &mut Frame, state: &AppState, config: &GrafConfig) {
    let area = frame.area();

    if state.show_help {
        draw_help(frame, area, config);
        return;
    }

    if !state.config_errors.is_empty() {
        draw_config_errors(frame, area, &state.config_errors);
        return;
    }

    if let Some(graph_state) = &state.graph_state {
        let guard = graph_state.read().unwrap_or_else(|e| e.into_inner());
        crate::graph::render::draw_graph_view(frame, &guard, config);
    }
}

fn draw_config_errors(frame: &mut Frame, area: Rect, errors: &[String]) {
    let config_path = crate::config::GrafConfig::config_path()
        .unwrap_or_default()
        .display()
        .to_string();
    let mut lines = vec!["Config Errors".to_string(), "".to_string()];
    for err in errors {
        lines.push(format!("  - {}", err));
        if let Some(suggestion) = suggest_fix(err) {
            lines.push(format!("    -> {}", suggestion));
        }
    }
    lines.push("".to_string());
    lines.push(format!("Fix: {}", config_path));
    lines.push("Press any key to close".to_string());

    let text = lines.join("\n");
    let paragraph = ratatui::widgets::Paragraph::new(text)
        .block(
            ratatui::widgets::Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .title("Config Error")
                .border_type(ratatui::widgets::BorderType::Rounded),
        )
        .alignment(ratatui::layout::Alignment::Left);

    let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0) + 4;
    let height = lines.len() as u16 + 2;
    let popup_area = ratatui::layout::Rect {
        x: (area.width.saturating_sub(max_width as u16)) / 2,
        y: (area.height.saturating_sub(height)) / 2,
        width: max_width.min(area.width as usize) as u16,
        height: height.min(area.height),
    };

    frame.render_widget(paragraph, popup_area);
}

fn suggest_fix(err: &str) -> Option<String> {
    let err_lower = err.to_lowercase();
    if err_lower.contains("theme") {
        return Some("Valid themes: default, tokyonight, catppuccinmocha, onedark, gruvbox, dracula, nord, solarizedlight, solarizeddark".to_string());
    }
    if err_lower.contains("background") {
        return Some("Valid backgrounds: transparent, solid".to_string());
    }
    if err_lower.contains("node_color_mode") {
        return Some("Valid modes: tag, folder, linkcount, uniform".to_string());
    }
    if err_lower.contains("edge_color_mode") {
        return Some("Valid modes: source, target, uniform".to_string());
    }
    if err_lower.contains("label_mode") {
        return Some("Valid modes: selected, neighbors, all, none".to_string());
    }
    if err_lower.contains("node_size_mode") {
        return Some("Valid modes: fixed, linkcount".to_string());
    }
    if err_lower.contains("border_style") {
        return Some("Valid styles: plain, rounded, double, none".to_string());
    }
    if err_lower.contains("legend_position") {
        return Some("Valid positions: topright, topleft, bottomright, bottomleft".to_string());
    }
    None
}

fn draw_help(frame: &mut Frame, area: Rect, _config: &GrafConfig) {
    let help_text = vec![
        "Keyboard",
        "  Arrows    Pan view",
        "  +/-       Zoom in/out",
        "  Enter     Open selected file",
        "  a         Auto-fit view",
        "  ?         Toggle help",
        "  q/Esc     Quit",
        "",
        "Mouse",
        "  Scroll    Zoom in/out",
        "  Drag bg   Pan view",
        "  Drag node Move node",
        "  Click     Select node",
        "  Dbl-click Open file",
    ];

    let text: String = help_text.join("\n");
    let paragraph = ratatui::widgets::Paragraph::new(text)
        .block(
            ratatui::widgets::Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .title("Help")
                .border_type(ratatui::widgets::BorderType::Rounded),
        )
        .alignment(ratatui::layout::Alignment::Left);

    let max_width = help_text.iter().map(|l| l.len()).max().unwrap_or(0) + 4;
    let help_height = help_text.len() as u16 + 2;
    let help_area = ratatui::layout::Rect {
        x: (area.width.saturating_sub(max_width as u16)) / 2,
        y: (area.height.saturating_sub(help_height)) / 2,
        width: max_width.min(area.width as usize) as u16,
        height: help_height.min(area.height),
    };

    frame.render_widget(paragraph, help_area);
}
