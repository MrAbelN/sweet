use bevy::prelude::*;
use extend::ext;

use crate::utility;

#[ext(name = OptI32X)]
pub impl App {
	fn __(&mut self) -> &mut Self { self }
	fn forky_surrender_focus(&mut self) -> &mut Self {
		self.add_startup_system(utility::surrender_focus);
		self
	}
	fn forky_exit_after(&mut self, secs: f64) -> &mut Self {
		self.add_system(utility::create_exit_after_system(secs));
		self
	}
}
