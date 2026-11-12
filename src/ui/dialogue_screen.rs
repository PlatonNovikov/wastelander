use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Layout},
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame
};

use crate::dialogue::{self, Answer, Dialogue, Node, load_dialogue};
use crate::game::{Game, VarMap};

use super::{Screen, ScreenReturn};

pub struct DialogueScreen {
    dialogue: Dialogue,
    current: String,
    selected: usize,
    // local variables of this dialogue screen
    locals: VarMap,
    sorted_answers: Vec<usize>,
}

impl Screen for DialogueScreen {
    fn draw(&self, frame: &mut Frame, game: &Game) {
        let node = self.current_node();
        let answers= self.list_answers(game);

        let block = Block::default().title("Dialogue").borders(Borders::ALL);
        let inner = block.inner(frame.area());
        frame.render_widget(block, frame.area());

        let [text_area, answers_area] = Layout::vertical([
            Constraint::Min(3),
            Constraint::Length(answers.len().max(1) as u16 + 1),
        ])
        .areas(inner);

        let text = Paragraph::new(node.text.as_str()).wrap(Wrap { trim: true });
        frame.render_widget(text, text_area);

        let lines: Vec<Line> = std::iter::once(Line::raw(""))
            .chain(answers.iter().enumerate().map(|(i, a)| {
                let text = &node.answers[*a].text;
                if i == answers[self.selected] {
                    Line::styled(format!("> {}", text), Style::new().add_modifier(Modifier::BOLD))
                } else {
                    Line::raw(format!("  {}", text))
                }
            }))
            .collect();

        frame.render_widget(Paragraph::new(lines), answers_area);
    }

    fn handle_input(&mut self, key: KeyEvent, game: &mut Game) -> ScreenReturn {
        let answers_len = self.sorted_answers.len();
        let node = self.dialogue.nodes.get(&self.current).unwrap();
        let Some(answer) = node.answers.get(self.sorted_answers[self.selected])
        else {
            return ScreenReturn::PopScreen;
        };

        match key.code {
            KeyCode::Char('h') => {
                game.hide_checks = !game.hide_checks;
            }

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
                for effect in (&answer.effects).into_iter() {
                    let _ = effect.calc_mut(game.player.as_ref().unwrap(), &mut self.locals, &mut game.globals);
                }

                if answer.end {
                    return ScreenReturn::PopScreen;
                }

                if let Some(next) = &answer.next {
                    self.current = next.clone();
                    self.selected = 0;
                    let _ = self.sort_answers(game);
                }
            }

            _ => {}
        }

        ScreenReturn::None
    }
}

impl DialogueScreen {
    pub fn new(dialogue_file: &str, game: &mut Game) -> Self {
        let dialogue = load_dialogue(dialogue_file).unwrap();
        let mut screen = Self { 
            dialogue,
            current: String::from("start"),
            selected: 0,
            locals: HashMap::new(),
            sorted_answers: Vec::new(),
        };
        let _ = screen.sort_answers(game);
        screen
    }

    fn current_node(&self) -> &Node {
        self.dialogue.nodes.get(&self.current).unwrap()
    }

    fn sort_answers(&mut self, game: &Game) -> Result<(), Box<dyn std::error::Error>> {
        let answers = &self.current_node().answers;
        let mut visible = Vec::new();
        for (i, answer) in answers.iter().enumerate() {
            let ok = match &answer.condition {
                Some(cond) => cond.calc_immut(&game.player.as_ref().unwrap(), &self.locals, &game.globals)? != 0,
                None => true,
            };
            if ok {
                visible.push(i);
            }
        }

        self.sorted_answers = visible;

        Ok(())
    }

    fn list_answers(&self, game: &Game) -> Vec<usize> {
        if game.hide_checks == true {
            (0..self.current_node().answers.len()).collect()
        } else {
            self.sorted_answers.clone()
        }
    }
}
