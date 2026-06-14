use ratatui::Frame;

use crate::app::{App, ConfirmAction, Modal};

pub fn render(frame: &mut Frame, app: &App) {
    match &app.modal {
        Some(Modal::Help) => {
            render_help(frame);
        }
        Some(Modal::Filter) => {
            render_filter(frame, app);
        }
        Some(Modal::Confirm(action)) => {
            render_confirm(frame, action);
        }
        Some(Modal::Details) => {
            render_details(frame, app);
        }
        None => {
            render_list(frame, app);
        }
    }
}

fn render_help(frame: &mut Frame) {
    use ratatui::layout::{Constraint, Layout, Margin};
    use ratatui::widgets::{Block, Borders, Clear, Paragraph};

    let popup_area = centered_rect(60, 25, frame.area());
    frame.render_widget(Clear, popup_area);
    frame.render_widget(
        Paragraph::new("Help\n\nq: quit\n?: help\n/: filter\n↑↓: scroll\nEnter: details\ns: start\nS: stop\nr: reboot")
            .block(Block::default().borders(Borders::ALL).title("Help")),
        popup_area.inner(Margin::new(1, 1)),
    );

    fn centered_rect(
        percent_x: u16,
        percent_y: u16,
        r: ratatui::layout::Rect,
    ) -> ratatui::layout::Rect {
        let popup_layout = Layout::vertical([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ]);
        let popup_area = popup_layout.split(r)[1];
        let horizontal_layout = Layout::horizontal([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ]);
        horizontal_layout.split(popup_area)[1]
    }
}

fn render_filter(frame: &mut Frame, app: &App) {
    use ratatui::widgets::{Block, Borders, Paragraph};

    let area = frame.area();
    frame.render_widget(
        Paragraph::new(format!("Filter: {}", app.filter))
            .block(Block::default().borders(Borders::ALL)),
        area,
    );
}

fn render_confirm(frame: &mut Frame, action: &ConfirmAction) {
    use ratatui::widgets::{Block, Borders, Paragraph};

    let area = frame.area();
    let msg = match action {
        ConfirmAction::Stop { node, vmid } => format!("Stop {} on {}? (y/n)", vmid, node),
        ConfirmAction::Reboot { node, vmid } => format!("Reboot {} on {}? (y/n)", vmid, node),
    };
    frame.render_widget(
        Paragraph::new(msg).block(Block::default().borders(Borders::ALL).title("Confirm")),
        area,
    );
}

fn render_details(frame: &mut Frame, app: &App) {
    use ratatui::widgets::{Block, Borders, Paragraph};

    let area = frame.area();
    let text = if let Some(r) = app.selected_resource() {
        format!(
            "Name: {}\nType: {}\nStatus: {}\nNode: {:?}\nCPU: {:?}\nMem: {:?}",
            r.name, r.r#type, r.status, r.node, r.cpu, r.mem
        )
    } else {
        "No resource selected".to_string()
    };
    frame.render_widget(
        Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Details")),
        area,
    );
}

fn render_list(frame: &mut Frame, app: &App) {
    use ratatui::widgets::{Block, Borders, Paragraph};

    let area = frame.area();
    let lines: String = app
        .filtered_resources()
        .iter()
        .enumerate()
        .map(|(i, r)| {
            let marker = if i == app.selected_index { "> " } else { "  " };
            format!("{}{} ({}): {}\n", marker, r.name, r.r#type, r.status)
        })
        .collect::<String>();
    let content = if lines.is_empty() {
        "No resources".to_string()
    } else {
        lines
    };
    frame.render_widget(
        Paragraph::new(content).block(Block::default().borders(Borders::ALL)),
        area,
    );
}
