use ratatui::widgets::Paragraph;
use ratatui::Frame;

pub fn render(frame: &mut Frame) {
    frame.render_widget(Paragraph::new(""), frame.area());
}
