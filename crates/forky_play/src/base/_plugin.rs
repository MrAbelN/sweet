use forky_core::{*, math::TAU};
use bevy::{
	prelude::*,
	window::{PresentMode, WindowDescriptor},
};
use bevy_inspector_egui::WorldInspectorPlugin;
use std::f32::consts::PI;

use super::CompanionCube;
use super::*;

pub struct BasicPlugin;

const CLEAR_COLOR: Color = Color::hsl(0.3 * 360., 1., 0.8);

impl Plugin for BasicPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(ClearColor(CLEAR_COLOR))
			.add_startup_system(spawn_basic_scene)
			.add_startup_system(spawn_cube)
			.add_startup_system(spawn_camera)
			.add_system(rotate_cube);
	}
}
