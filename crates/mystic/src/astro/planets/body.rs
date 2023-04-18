use strum_macros::{Display, EnumCount};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumCount)]
pub enum Body {
	Sun,
	Mercury,
	Venus,
	Earth,
	Moon,
	Mars,
	Jupiter,
	Saturn,
	Uranus,
	Neptune,
}