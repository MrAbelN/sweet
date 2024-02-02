use crate::tests::utils::expect_tree;
use bevy_app::App;
use gamai::prelude::*;
use sweet::*;

#[sweet_test]
pub fn works() -> Result<()> {
	let mut app = App::new();
	app.add_plugins(ActionPlugin::<BuiltinNode, _>::default());

	let target = app.world.spawn_empty().id();

	let action_graph =
		ActionTree::<BuiltinNode>::new(vec![SequenceSelector.into()])
			.with_leaf(vec![SetRunResult::success().into()])
			.with_leaf(vec![SetRunResult::failure().into()])
			.into_action_graph();

	let entity_graph = action_graph.spawn(&mut app.world, target);

	app.update();
	expect_tree(
		&mut app,
		&entity_graph,
		Tree::new(Some(&Running))
			.with_leaf(Some(&Running))
			.with_leaf(None),
	)?;

	app.update();
	expect_tree(
		&mut app,
		&entity_graph,
		Tree::new(Some(&Running)).with_leaf(None).with_leaf(None),
	)?;

	app.update();
	expect_tree(
		&mut app,
		&entity_graph,
		Tree::new(Some(&Running))
			.with_leaf(None)
			.with_leaf(Some(&Running)),
	)?;

	app.update();
	expect_tree(
		&mut app,
		&entity_graph,
		Tree::new(Some(&Running)).with_leaf(None).with_leaf(None),
	)?;

	app.update();
	expect_tree::<Running>(
		&mut app,
		&entity_graph,
		Tree::new(None).with_leaf(None).with_leaf(None),
	)?;
	expect_tree(
		&mut app,
		&entity_graph,
		Tree::new(Some(&RunResult::Failure))
			.with_leaf(None)
			.with_leaf(None),
	)?;

	Ok(())
}
