use bevy_app::prelude::*;
// use bevy_ecs::prelude::*;
use gamai::*;
use sweet::*;

#[sweet_test]
pub fn children() -> Result<()> {
	let my_tree = || {
		tree! {
			<parallel>
				<parallel>
					<empty_node/>
				</parallel>
			</parallel>
		}
	};

	let mut app = App::new();

	app.add_plugins(AiPlugin::new(my_tree));
	let entity = app.world.spawn(AiBundle::new(my_tree)).id();

	expect(my_tree.child(0).node_state(&app.world, entity)).to_be_none()?;
	expect(my_tree.child(0).child(0).node_state(&app.world, entity))
		.to_be_none()?;

	app.update();

	expect(my_tree.child(0).node_state(&app.world, entity))
		.to_be(Some(NodeState::Running))?;
	expect(my_tree.child(0).child(0).node_state(&app.world, entity))
		.to_be_none()?;

	app.update();
	expect(my_tree.child(0).node_state(&app.world, entity))
		.to_be(Some(NodeState::Running))?;
	expect(my_tree.child(0).child(0).node_state(&app.world, entity))
		.to_be(Some(NodeState::Running))?;

	Ok(())
}
