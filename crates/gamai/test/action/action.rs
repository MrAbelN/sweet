use super::*;
use bevy_app::App;
use gamai::prelude::*;
use sweet::*;


#[sweet_test]
pub fn default_components() -> Result<()> {
	let mut app = App::new();
	let target = app.world.spawn_empty().id();
	let actions = test_action_graph_typed();
	let entities = actions.spawn(&mut app.world, target);
	let entity = *entities.root().unwrap();

	expect(&app).to_have_component::<SetScore>(entity)?;
	expect(&app).to_have_component::<TargetEntity>(entity)?;
	expect(&app).to_have_component::<RunTimer>(entity)?;
	expect(&app).to_have_component::<Score>(entity)?;


	Ok(())
}

#[sweet_test]
pub fn sync_system() -> Result<()> {
	let mut app = App::new();
	app.add_plugins(ActionPlugin::<BuiltinNode, _>::default());

	let target = app.world.spawn_empty().id();
	let actions = test_action_graph_boxed();
	let entities = actions.spawn(&mut app.world, target);
	let entity = *entities.root().unwrap();


	app.world
		.entity_mut(entity)
		.insert(SetScore::new(Score::Pass));

	expect(&app).component(entity)?.to_be(&Score::Fail)?;
	app.update();
	expect(&app).component(entity)?.to_be(&Score::Pass)?;

	Ok(())
}
