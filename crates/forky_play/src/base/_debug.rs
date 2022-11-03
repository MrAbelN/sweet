use bevy::prelude::*;
use forky_core::math::*;


#[derive(Component)]
pub struct CompanionCube;


pub fn rotate_cube(time: Res<Time>, mut query: Query<&mut Transform, With<CompanionCube>>) {
	for mut transform in &mut query {
		transform.rotate_y(TAU * 0.1 * time.delta_seconds());
	}
}

pub fn spawn_cube(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands
		.spawn_bundle(PbrBundle {
			mesh: meshes.add(Mesh::from(shape::Cube { size: 1. })),
			material: materials.add(Color::rgb(0., 1., 1.).into()),
			transform: Transform::from_xyz(0., 1., 0.),
			..default()
		})
		.insert(CompanionCube);

}
