use ratatui::{DefaultTerminal, Frame};

mod screen; 
mod app;
mod game;

use crate::app::{App};

#[derive(Copy, Clone)]
struct Special {
	strenght: i8,
	perception: i8,
	endurance: i8,
	charisma: i8,
	intelligence: i8,
	agility: i8,
	luck: i8,
}

impl Default for Special {
	fn default() -> Self {
		Self {
			strenght: 5,
			perception: 5,
			endurance: 5,
			charisma: 5,
			intelligence: 5,
			agility: 5,
			luck: 5
		}
	}
}

impl Special {
	fn id_get(&self, id: usize) -> Option<i8> {
		match id {
			0 => Some(self.strenght),
			1 => Some(self.perception),
			2 => Some(self.endurance),
			3 => Some(self.charisma),
			4 => Some(self.intelligence),
			5 => Some(self.agility),
			6 => Some(self.luck),
			_ => None,
		}
	}

	fn id_get_mut(&mut self, id: usize) -> Option<&mut i8> {
		match id {
			0 => Some(&mut self.strenght),
			1 => Some(&mut self.perception),
			2 => Some(&mut self.endurance),
			3 => Some(&mut self.charisma),
			4 => Some(&mut self.intelligence),
			5 => Some(&mut self.agility),
			6 => Some(&mut self.luck),
			_ => None,
		}
	}
}

#[derive(Default, Copy, Clone)]
struct Skills {
	barter: i16,
	e_weapons: i16, //energy weapons
	explosives: i16,
	guns: i16,
	lockpick: i16,
	medicine: i16,
	m_weapons: i16, //melee weapons
	repair: i16,
	science: i16,
	sneak: i16,
	speech: i16,
	survival: i16,
	unarmed: i16,
}

impl Skills {
	fn from_special(special: &Special) -> Self {
		let calc = |stat: i8| -> i16 {
			(2 + (2 * stat) + (special.luck + 1) / 2).into()
		};
		Self {
			barter: calc(special.charisma),
			e_weapons: calc(special.perception),
			explosives: calc(special.perception),
			guns: calc(special.agility),
			lockpick: calc(special.perception),
			medicine: calc(special.intelligence),
			m_weapons: calc(special.strenght),
			repair: calc(special.intelligence),
			science: calc(special.intelligence),
			sneak: calc(special.agility),
			speech: calc(special.charisma),
			survival: calc(special.endurance),
			unarmed: calc(special.endurance),
		}
	}

	fn id_get(&self, id: usize) -> Option<i16> {
		match id {
			0 => Some(self.barter),
			1 => Some(self.e_weapons),
			2 => Some(self.explosives),
			3 => Some(self.guns),
			4 => Some(self.lockpick),
			5 => Some(self.medicine),
			6 => Some(self.m_weapons),
			7 => Some(self.repair),
			8 => Some(self.science),
			9 => Some(self.sneak),
			10 => Some(self.speech),
			11 => Some(self.survival),
			12 => Some(self.unarmed),
			_ => None
		}
	}

	fn id_get_mut(&mut self, id: usize) -> Option<&mut i16> {
		match id {
			0 => Some(&mut self.barter),
			1 => Some(&mut self.e_weapons), 
			2 => Some(&mut self.explosives),
			3 => Some(&mut self.guns),
			4 => Some(&mut self.lockpick),
			5 => Some(&mut self.medicine),
			6 => Some(&mut self.m_weapons),
			7 => Some(&mut self.repair),
			8 => Some(&mut self.science),
			9 => Some(&mut self.sneak),
			10 => Some(&mut self.speech),
			11 => Some(&mut self.survival),
			12 => Some(&mut self.unarmed),
			_ => None
		}
	}
}

struct Player {
	name: String,
	special: Special,
	skills: Skills,
}

fn main() -> color_eyre::Result<()> {
	color_eyre::install()?;
	let _ = ratatui::run(|terminal| App::default().run(terminal));
	Ok(())
}
