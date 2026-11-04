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

use crate::{Player, dialogue::{self, Dialogue, Node, load_dialogue}};
use crate::game::Game;

use super::{Screen, ScreenReturn};

pub struct DialogueScreen {
    dialogue: Dialogue,
    current: String,
    selected: usize,
}

impl Screen for DialogueScreen {
    fn draw(&self, frame: &mut Frame, game: &Game) {
        let node = self.current_node();

        let mut items = Vec::new();

        items.push(
            ListItem::new(node.text.clone())
        );

        items.push(
            ListItem::new("")
        );

        for (i, answer) in node.answers.iter().enumerate() {
            let cursor = if i == self.selected {
                "> "
            } else {
                "  "
            };

            items.push(
                ListItem::new(format!(
                    "{}{}",
                    cursor,
                    answer.text
                ))
            );
        }

        let list = List::new(items)
            .block(
                Block::default()
                    .title("Dialogue")
                    .borders(Borders::ALL)
            );

        frame.render_widget(list, frame.area());
    }

    fn handle_input(&mut self, key: KeyEvent, _game: &mut Game) -> ScreenReturn {
        let answers_len = self.current_node().answers.len();

        match key.code {
            KeyCode::Down => {
                if self.selected + 1 < answers_len {
                    self.selected += 1;
                }
            }

            KeyCode::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            }

            KeyCode::Enter => {
                let answer = &self.current_node().answers[self.selected];

                if answer.end {
                    return ScreenReturn::PopScreen;
                }

                if let Some(next) = &answer.next {
                    self.current = next.clone();
                    self.selected = 0;
                }
            }

            _ => {}
        }

        ScreenReturn::None
    }
}

impl DialogueScreen {
    pub fn new(dialogue_file: &str) -> Self {
        let dialogue = load_dialogue(dialogue_file).unwrap();
        Self { dialogue, 
            current: String::from("start"),
            selected: 0
        }
    }

    fn current_node(&self) -> &Node {
        self.dialogue.nodes.get(&self.current).unwrap()
    }
}