use ratatui::{
    Frame,
    widgets::{Block, Borders, List, ListItem},
};
use crossterm::event::{self, KeyEvent};

use crate::{Player, Skills, Special};

pub trait Screen: Default {
    fn draw(&self, frame: &mut Frame);
    fn handle_input(&mut self, key: KeyEvent);
}

enum CreationState {
    Name,
    Special,
    Skills,
}

pub struct PlayerCreation {
    name: Option<String>,
    special: Option<Special>,
    skills: Option<Skills>,
    state: CreationState,
}

impl Screen for PlayerCreation {
    fn draw(&self, frame: &mut Frame) {
        
    }

    fn handle_input(&mut self, key: KeyEvent) {
        
    }
}

impl Default for PlayerCreation {
    fn default() -> Self {
        Self {
            name: None,
            special: None,
            skills: None,
            state: None,
        }
    }
}

impl PlayerCreation {
    fn done(self) -> Player {
        Player {
            name: self.name.unwrap(),
            special: self.special.unwrap(),
            skills: self.skills.unwrap()
        }
    }
}