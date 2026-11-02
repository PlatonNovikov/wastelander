use ratatui::{DefaultTerminal, Frame};

mod ui; 
mod app;

use crate::app::{App};

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

#[derive(Default)]
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
}

struct Player {
	name: String,
	special: Special,
	skills: Skills,
}

fn select_special() -> Special {
	let mut special = Special::default();
	let mut selected: i8 = 0;
	loop {
		clearscreen::clear().expect("failed to clear screen");
		print!("Select your special stats:\n\n")

		for i in 0..7 {
			if selected == i {
				print!("> ");
			} else {
				print!("  ");
			}
			match i {
				0 => println!("strenght {}", special.strenght),
				1 => println!("perception {}", special.perception),
				2 => println!("endurance {}", special.endurance),
				3 => println!("charisma {}", special.charisma),
				4 => println!("intelligence {}", special.intelligence),
				5 => println!("agility {}", special.agility),
				6 => println!("luck {}", special.luck),
				_ => {}
			}
		}
	}
	special
}

fn make_player() -> Player {

	Skills::default().calc_skills(special);

}

fn main() -> color_eyre::Result<()> {
	color_eyre::install()?;
	ratatui::run(|terminal| App::default().run(terminal));
	Ok(())
}
