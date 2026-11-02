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
use crate::game::{Game};
use crate::ui::{Screen, PlayerCreation};

pub struct App {
    game: Game,
    screen: Box<dyn Screen>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            game: Game::default(),
            screen: Box::new(PlayerCreation::default()),
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            terminal.draw(|frame| self.screen.draw(frame))?;
            self.handle_events()?;
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.screen.handle_input(key_event);
            }
            _ => {}
        };
        Ok(())
    }
}