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

    if state.search_active {
        draw_search(frame, area, state, config);
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
        "  Arrows      Pan view",
        "  +/-         Zoom in/out",
        "  Enter       Open selected file",
        "  a           Auto-fit view",
        "  / or Ctrl+F Search nodes",
        "  ?           Toggle help",
        "  q/Esc       Quit",
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

fn draw_search(frame: &mut Frame, area: Rect, state: &AppState, config: &GrafConfig) {
    let colors = config.theme_colors();
    let max_visible = 10;
    let result_count = state.search_results.len();
    let visible_count = result_count.min(max_visible);
    let popup_width = 50.min(area.width.saturating_sub(4));
    let popup_height = (visible_count + 3).min(area.height.saturating_sub(4) as usize) as u16;

    let popup_x = (area.width.saturating_sub(popup_width)) / 2;
    let popup_y = 3;

    let popup_area = ratatui::layout::Rect::new(popup_x, popup_y, popup_width, popup_height);

    let input_line = format!("/ {}▎", state.search_query);

    let mut lines: Vec<ratatui::text::Line> = vec![ratatui::text::Line::styled(
        input_line,
        ratatui::style::Style::default().fg(colors.label_color),
    )];

    if result_count == 0 && !state.search_query.is_empty() {
        lines.push(ratatui::text::Line::styled(
            "  No matches",
            ratatui::style::Style::default().fg(colors.status_bar_color),
        ));
    } else {
        for (i, (_, title)) in state.search_results.iter().take(max_visible).enumerate() {
            let is_selected = i == state.search_selected;
            let style = if is_selected {
                ratatui::style::Style::default()
                    .fg(ratatui::style::Color::Black)
                    .bg(colors.label_color)
            } else {
                ratatui::style::Style::default().fg(colors.label_color)
            };
            let prefix = if is_selected { " > " } else { "   " };
            let display = truncate_display(title, (popup_width as usize).saturating_sub(6));
            lines.push(ratatui::text::Line::styled(
                format!("{}{}", prefix, display),
                style,
            ));
        }
    }

    let block = ratatui::widgets::Block::default()
        .borders(ratatui::widgets::Borders::ALL)
        .title("Search")
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(ratatui::style::Style::default().fg(colors.border_color));

    let paragraph = ratatui::widgets::Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, popup_area);
}

fn truncate_display(s: &str, max_len: usize) -> String {
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
