use bevy::prelude::*;
use forky_play::*;
fn main() {
	App::new()
		// .add_plugins(DefaultPlugins)
		.add_plugin(app::CustomDefaultPlugin)
		.add_plugin(app::SimplePlugin)
		.run();
}