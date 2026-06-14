use ratatui::{
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, TableState},
    Frame,
};

use crate::app::{App, ConfirmAction, Modal};

pub fn render(frame: &mut Frame, app: &App) {
    match &app.modal {
        Some(Modal::Help) => render_help(frame, app),
        Some(Modal::Filter) => render_filter(frame, app),
        Some(Modal::Confirm(action)) => render_confirm(frame, action, app),
        Some(Modal::Details) => render_details(frame, app),
        None => render_list(frame, app),
    }
}

fn render_help(frame: &mut Frame, _app: &App) {
    let popup_area = centered_rect(60, 25, frame.area());
    frame.render_widget(Clear, popup_area);
    frame.render_widget(
        Paragraph::new("Help\n\nq: quit\n?: help\n/: filter\n↑↓: scroll\nEnter: details\ns: start\nS: stop\nr: reboot")
            .block(Block::default().borders(Borders::ALL).title("Help")),
        popup_area.inner(Margin::new(1, 1)),
    );
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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

fn render_filter(frame: &mut Frame, app: &App) {
    let area = frame.area();
    frame.render_widget(
        Paragraph::new(format!("Filter: {}", app.filter))
            .block(Block::default().borders(Borders::ALL)),
        area,
    );
}

fn render_confirm(frame: &mut Frame, action: &ConfirmAction, _app: &App) {
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
    let no_color = app.config.no_color;

    let widths = [
        Constraint::Min(8),  // Type
        Constraint::Min(15), // Name
        Constraint::Min(10), // Node
        Constraint::Min(10), // Status
        Constraint::Min(8),  // CPU%
        Constraint::Min(12), // RAM
        Constraint::Min(12), // Disk
    ];

    let header = Row::new(vec![
        "Type", "Name", "Node", "Status", "CPU%", "RAM", "Disk",
    ])
    .style(if no_color {
        Style::default()
    } else {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    });

    let rows: Vec<Row> = app
        .display_resources
        .iter()
        .map(|r| {
            let status_style = if no_color {
                Style::default()
            } else {
                match r.status.as_str() {
                    "running" | "online" => Style::default().fg(Color::Green),
                    "stopped" => Style::default().fg(Color::Red),
                    _ => Style::default().fg(Color::Yellow),
                }
            };

            let cpu_str = r
                .cpu
                .map(|c| format!("{:.1}%", c * 100.0))
                .unwrap_or_else(|| "-".to_string());
            let ram_str = format_memory(r.mem, r.maxmem);
            let disk_str = format_disk(r.disk, r.maxdisk);

            Row::new(vec![
                Cell::from(r.r#type.clone()),
                Cell::from(r.name.clone()),
                Cell::from(r.node.clone().unwrap_or_default()),
                Cell::from(r.status.clone()).style(status_style),
                Cell::from(cpu_str),
                Cell::from(ram_str),
                Cell::from(disk_str),
            ])
        })
        .collect();

    let mut table_state = TableState::default();
    table_state.select(Some(app.selected_index));

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Resources"))
        .row_highlight_style(if no_color {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            Style::default().bg(Color::Blue).fg(Color::White)
        });

    frame.render_stateful_widget(table, frame.area(), &mut table_state);
}

fn format_memory(used: Option<u64>, total: Option<u64>) -> String {
    match (used, total) {
        (Some(u), Some(t)) => format!("{:.1} / {:.1} GB", u as f64 / 1e9, t as f64 / 1e9),
        _ => "-".to_string(),
    }
}

fn format_disk(used: Option<u64>, total: Option<u64>) -> String {
    match (used, total) {
        (Some(u), Some(t)) => {
            format!(
                "{} / {} GB",
                u / (1024 * 1024 * 1024),
                t / (1024 * 1024 * 1024)
            )
        }
        _ => "-".to_string(),
    }
}
