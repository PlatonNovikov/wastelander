use std::collections::HashMap;

pub type VarMap = HashMap<String, i16>;

#[derive(Default)]
pub struct Game {
    pub player: Option<Player>,
    pub globals: VarMap,
}

#[derive(Copy, Clone)]
pub struct Special {
	pub strenght: i8,
	pub perception: i8,
	pub endurance: i8,
	pub charisma: i8,
	pub intelligence: i8,
	pub agility: i8,
	pub luck: i8,
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
	pub const SPECIAL_NAMES: [&str; 7] = [
            "Strength",
            "Perception",
            "Endurance",
            "Charisma",
            "Intelligence",
            "Agility",
            "Luck",
    ];

	pub fn id_get(&self, id: usize) -> Option<i8> {
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

	pub fn id_get_mut(&mut self, id: usize) -> Option<&mut i8> {
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

	pub fn str_get(&self, name: &str) -> Option<i8> {
		match name {
			"strength" => Some(self.strenght),
			"perception" => Some(self.perception),
			"endurance" => Some(self.endurance),
			"charisma" => Some(self.charisma),
			"intelligence" => Some(self.intelligence),
			"agility" => Some(self.agility),
			"luck" => Some(self.luck),
			_ => None,
		}
	}
}

#[derive(Default, Copy, Clone)]
pub struct Skills {
	pub barter: i16,
	pub e_weapons: i16, //energy weapons
	pub explosives: i16,
	pub guns: i16,
	pub lockpick: i16,
	pub medicine: i16,
	pub m_weapons: i16, //melee weapons
	pub repair: i16,
	pub science: i16,
	pub sneak: i16,
	pub speech: i16,
	pub survival: i16,
	pub unarmed: i16,
}

impl Skills {
	pub const SKILLS_NAMES: [&str; 13] = [
            "barter",
	        "e_weapons",
	        "explosives",
	        "guns",
	        "lockpick",
	        "medicine",
	        "m_weapons",
	        "repair",
	        "science",
	        "sneak",
	        "speech",
	        "survival",
	        "unarmed",
        ];

	pub fn from_special(special: &Special) -> Self {
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

	pub fn id_get(&self, id: usize) -> Option<i16> {
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

	pub fn id_get_mut(&mut self, id: usize) -> Option<&mut i16> {
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

	pub fn str_get(&self, name: &str) -> Option<i16> {
		match name {
			"barter" => Some(self.barter),
			"e_weapons" => Some(self.e_weapons),
			"explosives" => Some(self.explosives),
			"guns" => Some(self.guns),
			"lockpick" => Some(self.lockpick),
			"medicine" => Some(self.medicine),
			"m_weapons" => Some(self.m_weapons),
			"repair" => Some(self.repair),
			"science" => Some(self.science),
			"sneak" => Some(self.sneak),
			"speech"  => Some(self.speech),
			"survival"  => Some(self.survival),
			"unarmed"  => Some(self.unarmed),
			_ => None
		}
	}
}

pub struct Player {
	pub name: String,
	pub special: Special,
	pub skills: Skills,
}

impl Player {
	pub fn stat_str(&self, name: &str) -> Option<i16> {
        self.skills.str_get(name)
            .or_else(|| self.special.str_get(name).map(i16::from))
    }
}