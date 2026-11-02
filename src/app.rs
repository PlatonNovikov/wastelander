use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

use crate::{Player};
use crate::ui::{Screen, PlayerCreation};

struct App<T: Screen> {
    player: Option<Player>,
    screen: T,
}

impl<T: Screen> Default for App<T> {
    fn default<T>() -> Self {
        Self {
            player: None,
            screen: PlayerCreation,
        }
    }
}

impl<T: Screen> App<T> {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        
    }
}