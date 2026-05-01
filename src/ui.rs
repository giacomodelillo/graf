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

    if let Some(graph_state) = &state.graph_state {
        let guard = graph_state.read().unwrap_or_else(|e| e.into_inner());
        crate::graph::render::draw_graph_view(frame, &guard, config);
    }
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
