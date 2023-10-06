use bevy_app::App;
use gamai::*;
use sweet::*;


#[tree_builder]
pub fn MyTree() -> impl AiTree {
	tree! {
		<sequence>
			<node_always_succeed/>
		</sequence>
	}
}


#[sweet_test]
pub fn it_works() -> Result<()> {
	let mut app = App::new();

	app.add_plugins(MyTree.plugin());

	let entity = app.world.spawn(MyTree.bundle()).id();

	expect(MyTree.node_state(&app.world, entity))
		.to_be(Some(NodeState::Running))?;

	app.update();
	app.update();

	expect(MyTree.node_state(&app.world, entity))
		.to_be(Some(NodeState::Success))?;

	Ok(())
}
