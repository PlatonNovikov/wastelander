use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    widgets::{Block, Borders, List, ListItem},
};

use crate::{game::Game, ui::{Screen, ScreenReturn, TodoScreen}};
use crate::ui::PlayerCreation;

#[derive(Default)]
pub struct MainMenu {
    selected: usize,
}

impl Screen for MainMenu {
    fn draw(&self, frame: &mut Frame, _game: &Game) {
        let items = [
            "New Game",
            "Load Game",
            "Settings",
            "Quit",
        ];

        let list: Vec<ListItem> = items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let cursor = if i == self.selected {
                    "> "
                } else {
                    "  "
                };

                ListItem::new(format!("{cursor}{item}"))
            })
            .collect();

        let widget = List::new(list)
            .block(
                Block::default()
                    .title("Main Menu")
                    .borders(Borders::ALL),
            );

        frame.render_widget(widget, frame.area());
    }


    fn handle_input(&mut self, key: KeyEvent, _game: &mut Game) -> ScreenReturn {
        match key.code {
            KeyCode::Down => {
                if self.selected < 3 {
                    self.selected += 1;
                }
            }

            KeyCode::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            }

            KeyCode::Enter => {
                match self.selected {
                    0 => {
                        return ScreenReturn::PushScreen(
                            Box::new(PlayerCreation::default())
                        );
                    }

                    1 => {
                        return ScreenReturn::PushScreen(
                            Box::new(TodoScreen::default())
                        );
                    }

                    2 => {
                        return ScreenReturn::PushScreen(
                            Box::new(TodoScreen::default())
                        );
                    }
                    3 => {
                        return ScreenReturn::Exit;
                    }

                    _ => {}
                }
            }

            _ => {}
        }

        ScreenReturn::None
    }
}