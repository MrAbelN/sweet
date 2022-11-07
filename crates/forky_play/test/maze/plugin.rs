use bevy::prelude::*;
use forky_core::{math::*, *};
use forky_play::{maze::*, *};
use sweet::*;
// use crate::maze::plugin::MazePlugin;
sweet! {

	it "works" {
		app::init()
			.add_plugin(maze::MazePlugin)
			// .add_startup_system(utility::surrender_focus)
			// .forky_exit_after(2)
			.run();

	}
}