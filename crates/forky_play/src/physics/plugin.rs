use super::*;
use crate::*;
use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.add_system(
				update_kinematic_bodies.in_base_set(CoreSet::PostUpdate),
			)
			.__();
	}
}
