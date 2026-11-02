use crossterm::event::{KeyEvent};
use ratatui::{Frame};
use crate::{Player};

pub trait Screen {
    fn draw(&self, frame: &mut Frame);
    fn handle_input(&mut self, key: KeyEvent) -> ScreenReturn;
}

pub enum ScreenReturn {
    None,
    ChangeScreen(Box<dyn Screen>),
    CreatedPlayer(Player),
}