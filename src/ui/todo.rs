use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::ui::{Screen, ScreenReturn};
use crate::{game::Game};

#[derive(Default)]
pub struct TodoScreen;

impl Screen for TodoScreen {
    fn draw(&self, frame: &mut Frame, _game: &Game) {
        let widget = Paragraph::new("This feature is not implemented yet. Press any key")
        .centered()
        .block(
            Block::default()
            .title("Not implemented")
            .borders(Borders::ALL)
        );
        frame.render_widget(widget, frame.area());
    }


    fn handle_input(&mut self, key: KeyEvent, _game: &mut Game) -> ScreenReturn {
        if key.is_press() {
            return ScreenReturn::PopScreen;
        }

        ScreenReturn::None
    }
}