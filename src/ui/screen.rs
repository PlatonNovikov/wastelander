use crossterm::event::{KeyEvent};
use ratatui::{Frame};
use crate::{
    game::*
};

pub trait Screen {
    fn draw(&self, frame: &mut Frame, game: &Game);
    fn handle_input(&mut self, key: KeyEvent, game: &mut Game) -> ScreenReturn;
}

pub enum ScreenReturn {
    None,
    PushScreen(Box<dyn Screen>),
    ReplaceScreen(Box<dyn Screen>),
    PopScreen,
    Exit,
}

pub trait SubScreen<T> {
    fn draw(&self, frame: &mut Frame);
    fn handle_input(&mut self, key: KeyEvent) -> T;
}