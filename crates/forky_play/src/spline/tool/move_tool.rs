use super::*;

use bevy::prelude::*;



pub fn move_items(
	camera_ray: Res<CameraRay>,
	keys: Res<Input<KeyCode>>,
	mouse: Res<Input<MouseButton>>,
	settings: Res<InteractionSettings>,
	mut selected_query: Query<&mut Transform, With<Selected>>,
) {
	for mut transform in selected_query.iter_mut() {
		if keys.just_pressed(KeyCode::PageUp) {
			transform.translation.y += settings.height_delta;
		}
		if keys.just_pressed(KeyCode::PageDown) {
			transform.translation.y -= settings.height_delta;
		}
		if let Some(intersect) = &camera_ray.entity_intersect {
			if mouse.pressed(SELECT_BUTTON) {
				transform.translation += intersect.delta;
			}
		}
	}
}
