use ratatui::{DefaultTerminal, Frame};

mod ui;
mod app;
mod game;
mod dialogue;

use crate::app::{App};

fn main() -> color_eyre::Result<()> {
	color_eyre::install()?;
	let _ = ratatui::run(|terminal| App::default().run(terminal));
	Ok(())
}
