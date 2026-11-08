use std::{io, process::exit};

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

use crate::game::{Game, Player};
use crate::ui::{Screen, PlayerCreation, ScreenReturn, MainMenu, DialogueScreen};
use crate::dialogue::{Dialogue, load_dialogue};

pub struct App {
    game: Game,
    screen: Vec<Box<dyn Screen>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            game: Game::default(),
            screen: vec![Box::new(MainMenu::default())],
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            terminal.draw(|frame| self.screen.last().unwrap().draw(frame, &self.game))?;
            self.handle_events()?;
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match self.screen.last_mut().unwrap().handle_input(key_event, &mut self.game) {
                    ScreenReturn::PushScreen(screen) => self.screen.push(screen),
                    ScreenReturn::ReplaceScreen(screen) => {
                        let _ = self.screen.pop();
                        self.screen.push(screen);
                    }
                    ScreenReturn::PopScreen => {
                        self.screen.pop();
                    }
                    ScreenReturn::Exit => {exit(0)}
                    ScreenReturn::None => {}
                }
            }
            _ => {}
        };
        Ok(())
    }
}