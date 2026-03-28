use std::io::{stdin, stdout, Write, Stdout, Stdin};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

#[derive(Copy, Clone)]
enum ESpecial {
	Strength,
	Perception,
	Endurance,
	Charisma,
	Intelligence,
	Agility,
	Luck,
}

//EWeapon - Energy Weapons
//MWeapon - Melee Weapons
#[derive(Copy, Clone)]
enum ESkills {
	Barter,
	EWeapons,
	Explosives,
	Guns,
	Lockpick,
	Medicine,
	MWeapons,
	Repair,
	Science,
	Sneak,
	Speech,
	Survival,
	Unarmed,
}

struct SSpecial {
	strength:		u8,
	perception:		u8,
	endurance:		u8,
	charisma: 		u8,
	intelligence:	u8,
	agility:		u8,
	luck:			u8,
}

impl SSpecial {
	fn init() -> SSpecial {
		SSpecial {
			strength:		5,
			perception:		5,
			endurance:		5,
			charisma: 		5,
			intelligence:	5,
			agility:		5,
			luck:			5,
		}
	}

	//get from an enum
	fn en_get(&self, en: ESpecial) -> u8 {
		match en {
			ESpecial::Strength => self.strength,
			ESpecial::Perception => self.perception,
			ESpecial::Endurance => self.endurance,
			ESpecial::Charisma => self.charisma,
			ESpecial::Intelligence => self.intelligence,
			ESpecial::Agility => self.agility,
			ESpecial::Luck => self.luck,
		}
	}

}

struct Skill {
	core_stat:	ESpecial, //core SPECIAL stat for the skill
	base_stat:	u8,	//permament skill points determed by base stats, level ups, and other things
}

impl Skill {
	fn init(core_stat: ESpecial, special: &SSpecial) -> Skill {
		Skill {
			core_stat,
			base_stat: Skill::calc(core_stat, special),
		}
	}

	fn recalc(&self, special: &SSpecial) {
		self.base_stat = Skill::calc(self.core_stat, special);
	}

	fn calc(core_stat: ESpecial, special: &SSpecial) -> u8 {
		2 + (2 * special.en_get(core_stat)) + (special.luck / 2)
	}
}

//eweapon - Energy Weapons
//mweapon - Melee Weapons
struct SSkills {
	barter:		Skill,
	eweapons:	Skill,
	explosives:	Skill,
	guns:		Skill,
	lockpick:	Skill,
	medicine:	Skill,
	mweapons:	Skill,
	repair:		Skill,
	science:	Skill,
	sneak:		Skill,
	speech:		Skill,
	survival:	Skill,
	unarmed:	Skill,
}

impl SSkills {
	fn init(special: &SSpecial) -> SSkills {
		SSkills {
			barter:		Skill::init(ESpecial::Charisma, special),
			eweapons:	Skill::init(ESpecial::Perception, special),
			explosives:	Skill::init(ESpecial::Perception, special),
			guns:		Skill::init(ESpecial::Agility, special),
			lockpick:	Skill::init(ESpecial::Perception, special),
			medicine:	Skill::init(ESpecial::Intelligence, special),
			mweapons:	Skill::init(ESpecial::Strength, special),
			repair:		Skill::init(ESpecial::Intelligence, special),
			science:	Skill::init(ESpecial::Intelligence, special),
			sneak:		Skill::init(ESpecial::Agility, special),
			speech:		Skill::init(ESpecial::Charisma, special),
			survival:	Skill::init(ESpecial::Endurance, special),
			unarmed:	Skill::init(ESpecial::Endurance, special),
		}
	}

	//get from an enum
	fn get(&self, en: ESkills) -> &Skill {
		match en {
			ESkills::Barter => &self.barter,
			ESkills::EWeapons => &self.eweapons,
			ESkills::Explosives => &self.explosives,
			ESkills::Guns => &self.guns,
			ESkills::Lockpick => &self.lockpick,
			ESkills::Medicine => &self.medicine,
			ESkills::MWeapons => &self.mweapons,
			ESkills::Repair => &self.repair,
			ESkills::Science => &self.science,
			ESkills::Sneak => &self.sneak,
			ESkills::Speech => &self.speech,
			ESkills::Survival => &self.survival,
			ESkills::Unarmed => &self.unarmed,
		}
	}

	fn recalc(&self, special: &SSpecial){
		self.barter.recalc(special);
		self.eweapons.recalc(special);
		self.explosives.recalc(special);
		self.guns.recalc(special);
		self.lockpick.recalc(special);
		self.medicine.recalc(special);
		self.mweapons.recalc(special);
		self.repair.recalc(special);
		self.science.recalc(special);
		self.sneak.recalc(special);
		self.speech.recalc(special);
		self.survival.recalc(special);
		self.unarmed.recalc(special);
	}
}

struct Player {
	name: String,
	special: SSpecial,
	skills: SSkills,
}

fn change_stat(s: &mut SSpecial, stat: ESpecial, delta: i8, bank: &mut i8) {
	let val = match stat {
		ESpecial::Strength => &mut s.strength,
		ESpecial::Perception => &mut s.perception,
		ESpecial::Endurance => &mut s.endurance,
		ESpecial::Charisma => &mut s.charisma,
		ESpecial::Intelligence => &mut s.intelligence,
		ESpecial::Agility => &mut s.agility,
		ESpecial::Luck => &mut s.luck,
	};

	if ((*bank == 0) && (delta > 0)) ||
		((*val == 10) && (delta > 0)) ||
		((*val == 1) && (delta < 0)) {
		return
	}

	let new = (*val as i8 + delta).clamp(1, 10);
	*val = new as u8;
	*bank -= delta;
}

fn select_special(
	p: &mut Player,
	stdout: &mut RawTerminal<Stdout>,
	stdin: &Stdin,
) {
	let mut selected: usize = 0;
	let mut bank: i8 = 5;

	let stats = [
		ESpecial::Strength,
		ESpecial::Perception,
		ESpecial::Endurance,
		ESpecial::Charisma,
		ESpecial::Intelligence,
		ESpecial::Agility,
		ESpecial::Luck,
	];

	let mut keys = stdin.lock().keys();

	loop {
		write!(
			stdout,
			"{}{}Use arrows to adjust, Enter to confirm\r\n",
			termion::clear::All,
			termion::cursor::Goto(1, 1),
		).unwrap();

		for (i, stat) in stats.iter().enumerate() {
			let value = p.special.en_get(*stat);

			if i == selected {
				write!(stdout, "> ").unwrap();
			} else {
				write!(stdout, "  ").unwrap();
			}

			let name = match stat {
				ESpecial::Strength => "Strength",
				ESpecial::Perception => "Perception",
				ESpecial::Endurance => "Endurance",
				ESpecial::Charisma => "Charisma",
				ESpecial::Intelligence => "Intelligence",
				ESpecial::Agility => "Agility",
				ESpecial::Luck => "Luck",
			};

			write!(stdout, "{}: {}\r\n", name, value).unwrap();
		}
		write!(stdout, "You have {} points left\r\n", bank);
		stdout.flush().unwrap();

		match keys.next().unwrap().unwrap() {
			Key::Up => {
				if selected > 0 {
					selected -= 1;
				}
			}
			Key::Down => {
				if selected < stats.len() - 1 {
					selected += 1;
				}
			}
			Key::Left => {
				change_stat(&mut p.special, stats[selected], -1, &mut bank);
			}
			Key::Right => {
				change_stat(&mut p.special, stats[selected], 1, &mut bank);
			}
			Key::Char('\n') => break,
			_ => {}
		}
	}
}

fn select_skills(
	p: &mut Player,
	stdout: &mut RawTerminal<Stdout>,
	stdin: &Stdin,
) {
	
}

fn make_character(	p: &mut Player,
					stdout: &mut RawTerminal<Stdout>,
					stdin: &Stdin) {
	write!(
		stdout,
		"{}{}Enter your name:{}",
		termion::clear::All,
		termion::cursor::Goto(1, 1),
		termion::cursor::Hide
	)
	.unwrap();
	stdout.flush().unwrap();

	let mut name = String::new();

	for key in stdin.lock().keys() {
		match key.unwrap() {
			Key::Char('\n') => break,
			Key::Char(c) => {
				name.push(c);
				write!(stdout, "{}", c).unwrap(); // echo character
			}
			Key::Backspace => {
				name.pop();
				write!(stdout, "\x08 \x08").unwrap(); // erase char visually
			}
			_ => {}
		}
		stdout.flush().unwrap();
	}

	p.name = name;

	write!(
		stdout,
		"{}{}Name: {}\n",
		termion::clear::All,
		termion::cursor::Goto(1, 1),
		p.name
	)
	.unwrap();
	stdout.flush().unwrap();

	select_special(p, stdout, stdin);
}

fn main() {
	let stdin = stdin();
	let mut stdout = stdout().into_raw_mode().unwrap();

	let mut special: SSpecial = SSpecial::init();
	let mut skills: SSkills =	SSkills::init(&special);

	let mut p: Player = Player {name: String::new(),
								special,
								skills};

	make_character(&mut p, &mut stdout, &stdin);
}
