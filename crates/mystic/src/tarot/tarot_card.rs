use super::{constants::TarotCardEnum, *};
use crate::astro::{chart::Sign, planets::Planet};
use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TarotCard {
	Minor(MinorArcana),
	Major(MajorArcana),
}
impl Display for TarotCard {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			TarotCard::Minor(minor) => Display::fmt(&minor, f),
			TarotCard::Major(major) => Display::fmt(&major, f),
		}
	}
}

impl Into<usize> for &TarotCard {
	fn into(self) -> usize {
		let value: TarotCardEnum = self.into();
		value.into()
	}
}


impl TryFrom<usize> for TarotCard {
	type Error = ();
	fn try_from(value: usize) -> Result<Self, Self::Error> {
		TarotCardEnum::try_from(value).map(|item| item.into())
	}
}


impl From<Planet> for TarotCard {
	fn from(planet: Planet) -> Self {
		match planet {
			Planet::Sun => TarotCard::THE_SUN,
			Planet::Moon => TarotCard::THE_HIGH_PRIESTESS,
			Planet::Mercury => TarotCard::THE_MAGICIAN,
			Planet::Venus => TarotCard::THE_EMPRESS,
			Planet::Mars => TarotCard::THE_TOWER,
			Planet::Jupiter => TarotCard::THE_WHEEL_OF_FORTUNE,
			Planet::Saturn => TarotCard::THE_WORLD,
			Planet::Uranus => TarotCard::THE_FOOL,
			Planet::Neptune => TarotCard::THE_HANGED_MAN,
			Planet::Pluto => TarotCard::JUDGMENT,
			//debatable
			Planet::Earth => TarotCard::ACE_OF_PENTACLES,
		}
	}
}

impl From<Sign> for TarotCard {
	fn from(sign: Sign) -> Self {
		match sign {
			Sign::Aries => TarotCard::THE_EMPEROR,
			Sign::Taurus => TarotCard::THE_HIEROPHANT,
			Sign::Gemini => TarotCard::THE_LOVERS,
			Sign::Cancer => TarotCard::THE_CHARIOT,
			Sign::Leo => TarotCard::STRENGTH,
			Sign::Virgo => TarotCard::THE_HERMIT,
			Sign::Libra => TarotCard::JUSTICE,
			Sign::Scorpio => TarotCard::DEATH,
			Sign::Sagittarius => TarotCard::TEMPERANCE,
			Sign::Capricorn => TarotCard::THE_DEVIL,
			Sign::Aquarius => TarotCard::THE_STAR,
			Sign::Pisces => TarotCard::THE_MOON,
		}
	}
}

// impl From<u8> for TarotCard {
// 	fn from(index: u8) -> Self {
// 		if index < NUM_MAJOR_ARCANA {
// 			TarotCard::Major(MajorArcana::from(index))
// 		} else {
// 			TarotCard::Minor(MinorArcana::from(index - NUM_MAJOR_ARCANA))
// 		}
// 	}
// }
