use std::{any::Any, hint::select_unpredictable};

use ratatui::{
    Frame,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use crossterm::event::{self, KeyEvent, KeyCode, KeyModifiers};

use crate::{Player, Skills, Special};

pub trait Screen {
    fn draw(&self, frame: &mut Frame);
    fn handle_input(&mut self, key: KeyEvent) -> ScreenReturn;
}

pub enum ScreenReturn {
    None,
    ChangeScreen(Box<dyn Screen>),
    CreatedPlayer(Player),
}

#[derive(Default)]
enum CreationState {
    #[default]
    Name,

    Special,
    Skills,
    Done,
}

#[derive(Default)]
pub struct PlayerCreation {
    name: String,
    special: Special,
    skills: Skills,
    state: CreationState,

    select: usize,
    points: i8,
}

impl Screen for PlayerCreation {
    fn draw(&self, frame: &mut Frame) {
        match self.state {
            CreationState::Name => self.name_draw(frame),
            CreationState::Special => self.special_draw(frame),
            CreationState::Skills => todo!(),
            CreationState::Done => todo!()
        }
    }

    fn handle_input(&mut self, key: KeyEvent) -> ScreenReturn {
        match self.state {
            CreationState::Name => self.name_input(key),
            CreationState::Special => self.special_input(key),
            CreationState::Skills => todo!(),
            CreationState::Done => todo!()
        };
        ScreenReturn::None
    }
}

impl PlayerCreation {
    fn name_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => {
                if !key.modifiers.contains(KeyModifiers::CONTROL) {
                    self.name.push(c);
                }
            }

            KeyCode::Backspace => {
                self.name.pop();
            }

            KeyCode::Enter => {
                if !self.name.trim().is_empty() {
                    self.state = CreationState::Special;
                    self.points = 5;
                    self.select = 0;
                }
            }
            _ => {}
        }
    }

    fn name_draw(&self, frame: &mut Frame) {
        let text = Paragraph::new(format!("Name: {}", self.name));
        frame.render_widget(text, frame.area());
    }


    fn special_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Down => {
                if self.select < 6 {
                    self.select += 1;
                }
            }

            KeyCode::Up => {
                if self.select > 0 {
                    self.select -= 1;
                }
            }

            KeyCode::Right => {
                let stat: &mut i8 = &mut self.special.id_get_mut(self.select).unwrap();
                if *stat < 10 && self.points > 0 {
                    *stat += 1;
                    self.points -= 1;
                } 
            }

            KeyCode::Left => {
                let stat: &mut i8 = self.special.id_get_mut(self.select).unwrap();
                if *stat > 1 {
                    *stat -= 1;
                    self.points += 1;
                } 
            }

            KeyCode::Enter => {
                if self.points == 0 {
                    self.select = 0;
                    self.points = 3;
                    self.skills = Skills::from_special(&self.special);
                    self.state = CreationState::Skills;
                }
            }
            _ => {}
        }
    }

    fn special_draw(&self, frame: &mut Frame) {
        let names = [
            "Strength",
            "Perception",
            "Endurance",
            "Charisma",
            "Intelligence",
            "Agility",
            "Luck",
        ];

        let items: Vec<ListItem> = names
            .iter()
            .enumerate()
            .map(|(i, name)| {
                let value = self.special.id_get(i).unwrap();

                let prefix = if i == self.select {
                    "> "
                } else {
                    "  "
                };

                ListItem::new(format!("{prefix}{name:<13} {value}"))
            })
            .collect();

        let list = List::new(items).block(
            Block::default()
                .title(format!("SPECIAL ({} points left)", self.points))
                .borders(Borders::ALL),
        );

        frame.render_widget(list, frame.area());
    }

    fn skills_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Down => {
                if self.select < 6 {
                    self.select += 1;
                }
            }

            KeyCode::Up => {
                if self.select > 0 {
                    self.select -= 1;
                }
            }

            KeyCode::Right => {
                let stat: &mut i8 = &mut self.special.id_get_mut(self.select).unwrap();
                if *stat < 10 && self.points > 0 {
                    *stat += 1;
                    self.points -= 1;
                } 
            }

            KeyCode::Left => {
                let stat: &mut i8 = self.special.id_get_mut(self.select).unwrap();
                if *stat > 1 {
                    *stat -= 1;
                    self.points += 1;
                } 
            }

            KeyCode::Enter => {
                if self.points == 0 {
                    self.select = 0;
                    self.points = 3;
                    self.skills = Skills::from_special(&self.special);
                    self.state = CreationState::Skills;
                }
            }
            _ => {}
        }
    }

    fn skills_draw(&self, frame: &mut Frame) {
        let names = [
            "Strength",
            "Perception",
            "Endurance",
            "Charisma",
            "Intelligence",
            "Agility",
            "Luck",
        ];

        let items: Vec<ListItem> = names
            .iter()
            .enumerate()
            .map(|(i, name)| {
                let value = self.special.id_get(i).unwrap();

                let prefix = if i == self.select {
                    "> "
                } else {
                    "  "
                };

                ListItem::new(format!("{prefix}{name:<13} {value}"))
            })
            .collect();

        let list = List::new(items).block(
            Block::default()
                .title(format!("SPECIAL ({} points left)", self.points))
                .borders(Borders::ALL),
        );

        frame.render_widget(list, frame.area());
    }
}