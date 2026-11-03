use crossterm::event::{KeyEvent};
use ratatui::{Frame};
use crate::{Player};

pub trait Screen {
    fn draw(&self, frame: &mut Frame);
    fn handle_input(&mut self, key: KeyEvent) -> ScreenReturn;
}

pub enum ScreenReturn {
    None,
    PushScreen(Box<dyn Screen>),
    ReplaceScreen(Box<dyn Screen>),
    PopScreen,
    CreatedPlayer(Player),
    Exit,
}

pub trait SubScreen<T> {
    fn draw(&self, frame: &mut Frame);
    fn handle_input(&mut self, key: KeyEvent) -> T;
}