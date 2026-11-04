use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout},
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame
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
    fn draw(&self, frame: &mut Frame, _game: &Game) {
        let node = self.current_node();

        let block = Block::default().title("Dialogue").borders(Borders::ALL);
        let inner = block.inner(frame.area());
        frame.render_widget(block, frame.area());

        let [text_area, answers_area] = Layout::vertical([
            Constraint::Min(3),
            Constraint::Length(node.answers.len().max(1) as u16 + 1),
        ])
        .areas(inner);

        let text = Paragraph::new(node.text.as_str()).wrap(Wrap { trim: true });
        frame.render_widget(text, text_area);

        let lines: Vec<Line> = std::iter::once(Line::raw(""))
            .chain(node.answers.iter().enumerate().map(|(i, a)| {
                if i == self.selected {
                    Line::styled(format!("> {}", a.text), Style::new().add_modifier(Modifier::BOLD))
                } else {
                    Line::raw(format!("  {}", a.text))
                }
            }))
            .collect();

        frame.render_widget(Paragraph::new(lines), answers_area);
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