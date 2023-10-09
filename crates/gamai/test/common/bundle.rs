use bevy_app::App;
use gamai::*;
use sweet::*;

#[sweet_test]
pub fn root() -> Result<()> {
	let my_tree = tree! {<empty_node/>};

	let mut app = App::new();

	let entity = app.world.spawn(my_tree.bundle_inactive()).id();

	expect(my_tree.node_state(&app.world, entity)).to_be_none()?;
	expect(my_tree.edge_state(&app.world, entity))
		.to_be(Some(EdgeState::Fail))?;

	let entity = app.world.spawn(my_tree.bundle()).id();
	expect(my_tree.node_state(&app.world, entity))
		.to_be(Some(NodeState::Running))?;

	Ok(())
}