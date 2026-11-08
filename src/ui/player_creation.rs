use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget, Borders, List, ListItem},
    DefaultTerminal, Frame,
};

use super::{Screen, ScreenReturn};
use crate::{
    game::{Game, Player, Skills, Special},
    ui::{DialogueScreen, screen::SubScreen}
};

enum SubScreenResult {
    None,
    Name(String),
    Special(Special),
    Skills(Skills),
}

pub struct PlayerCreation {
    name: String,
    special: Special,
    skills: Skills,
    submenu: Box<dyn SubScreen<SubScreenResult>>,
}

impl Default for PlayerCreation {
    fn default() -> Self {
        Self {
            name: String::new(),
            special: Special::default(),
            skills: Skills::default(),
            submenu: Box::new(NameCreation::default()),
        }
    }
}

impl Screen for PlayerCreation {
    fn draw(&self, frame: &mut Frame, _game: &Game) {
        self.submenu.draw(frame);
    }

    fn handle_input(&mut self, key: KeyEvent, game: &mut Game) -> ScreenReturn {
        match self.submenu.handle_input(key) {
            SubScreenResult::None => {},
            SubScreenResult::Name(name) => {
                self.name = name;
                self.submenu = Box::new(SpecialCreation::default());
            }
            SubScreenResult::Special(special) => {
                self.special = special;
                self.submenu = Box::new(SkillsCreation::new(&special));
            }
            SubScreenResult::Skills(skills) => {
                game.player = Some(Player {
                    name: self.name.clone(),
                    special: self.special,
                    skills
                });
                let next = Box::new(DialogueScreen::new("dialogues/test.toml"));
                return ScreenReturn::ReplaceScreen(next);
            }
        }
        return ScreenReturn::None
    }
}

#[derive(Default)]
struct NameCreation {
    name: String,
}

impl SubScreen<SubScreenResult> for NameCreation {
    fn draw(&self, frame: &mut Frame) {
        let text = Paragraph::new(self.name.clone()).block(
            Block::default()
                .title("Enter your name")
                .borders(Borders::ALL),
        );
        frame.render_widget(text, frame.area());
    }

    fn handle_input(&mut self, key: KeyEvent) -> SubScreenResult {
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
                return SubScreenResult::Name(self.name.clone());
            }
            _ => {}
        }
        SubScreenResult::None
    }
}

struct SpecialCreation {
    special: Special,
    select: usize,
    points: i8,
}

impl Default for SpecialCreation {
    fn default() -> Self {
        SpecialCreation {
            special: Special::default(),
            select: 0,
            points: 5
        }
    }
}

impl SubScreen<SubScreenResult> for SpecialCreation {
    fn draw(&self, frame: &mut Frame) {
        let names = Special::SPECIAL_NAMES;

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

    fn handle_input(&mut self, key: KeyEvent) -> SubScreenResult {
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
                let stat: &mut i8 = self.special.id_get_mut(self.select).unwrap();
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
                return SubScreenResult::Special(self.special);
                }

            _ => {}
        }
        SubScreenResult::None
    }
}

struct SkillsCreation {
    skills: Skills,
    select: usize,
    points: i8,
    selected_skills: Vec<usize>,
}

impl SkillsCreation {
    fn new(special: &Special) -> Self {
        SkillsCreation {
            skills: Skills::from_special(special),
            select: 0,
            points: 3,
            selected_skills: Vec::new()
        }
    }
}

impl SubScreen<SubScreenResult> for SkillsCreation {
    fn draw(&self, frame: &mut Frame) {
        let names = Skills::SKILLS_NAMES;

        let items: Vec<ListItem> = names
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let value = self.skills.id_get(i).unwrap();

            let cursor = if i == self.select { ">" } else { " " };
            let picked = if self.selected_skills.contains(&i) { "*" } else { " " };

            ListItem::new(format!(
                "{cursor} [{picked}] {name:<16} {value}"
            ))
        })
        .collect();

        let list = List::new(items).block(
            Block::default()
            .title(format!(
                "Tag Skills ({} left)",
                self.points
            ))
            .borders(Borders::ALL),
        );

        frame.render_widget(list, frame.area());
    }

    fn handle_input(&mut self, key: KeyEvent) -> SubScreenResult {
        match key.code {
            KeyCode::Down => {
                if self.select < 12 {
                    self.select += 1;
                }
            }

            KeyCode::Up => {
                if self.select > 0 {
                    self.select -= 1;
                }
            }

            KeyCode::Char(c) => {
                match c {
                    ' ' => {
                        if self.points > 0 && !self.selected_skills.contains(&self.select) {
                            self.points -= 1;
                            *self.skills.id_get_mut(self.select).unwrap() += 15;
                            self.selected_skills.push(self.select);
                        }
                        else if self.selected_skills.contains(&self.select){
                            self.points += 1;
                            *self.skills.id_get_mut(self.select).unwrap() -= 15;
                            self.selected_skills.retain(|val| *val != self.select);
                        }
                    }
                    _ => {}
                }
            }

            KeyCode::Enter => {
                if self.points == 0 {
                    return SubScreenResult::Skills(self.skills);
                }
            }
            _ => {}
        }
        SubScreenResult::None
    }
}
