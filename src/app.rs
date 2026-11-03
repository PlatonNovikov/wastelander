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

use crate::{Player};
use crate::game::{Game};
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
            terminal.draw(|frame| self.last_screen().draw(frame))?;
            self.handle_events()?;
        }
    }
    
    fn last_screen(&mut self) -> &mut Box<dyn Screen> {
        self.screen.last_mut().unwrap()
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match self.last_screen().handle_input(key_event) {
                    ScreenReturn::PushScreen(screen) => self.screen.push(screen),
                    ScreenReturn::ReplaceScreen(screen) => {
                        let _ = self.screen.pop();
                        self.screen.push(screen);
                    }
                    ScreenReturn::PopScreen => {
                        self.screen.pop();
                        if self.screen.len() == 0 {
                            exit(0);
                        }
                    }
                    ScreenReturn::CreatedPlayer(player) => {
                        self.game.player = Some(player);
                        self.screen.pop();
                        self.screen.push(Box::new(DialogueScreen::new("src/dialogue/dialogues/test.toml")));
                    },
                    ScreenReturn::Exit => {exit(0)}
                    ScreenReturn::None => {}
                }
            }
            _ => {}
        };
        Ok(())
    }
}