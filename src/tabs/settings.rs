use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

#[derive(Debug, Default)]
pub struct SettingsTab {}

impl Widget for &SettingsTab {
    fn render(self, area: Rect, buf: &mut Buffer) {}
}

impl SettingsTab {
    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        Ok(())
    }
}
