use bevy_app::prelude::*;
use gamai::common_actions::*;
use gamai::common_selectors::*;
use gamai::*;
use sweet::*;

#[sweet_test]
pub fn works() -> Result<()> {
	let my_tree = || {
		tree! {
			<sequence apply_deferred>
				<node_always_succeed apply_deferred/>
				<node_always_succeed apply_deferred/>
				// <node_always_succeed apply_deferred/>
				// <node_always_succeed apply_deferred/>
			</sequence>
		}
	};

	let mut app = App::new();

	app.add_plugins(TreePlugin::new(my_tree));
	let entity = app.world.spawn(TreeBundle::new(my_tree)).id();

	app.update();

	let out = PropTree::<Running>::new(my_tree, &app.world, entity);
	expect(out.value).to_be_some()?;
	let out = PropTree::<ActionResult>::new(my_tree, &app.world, entity);
	expect(out.children[0].value).to_be_some()?;
	expect(out.children[1].value).to_be_none()?;

	app.update();

	let out = PropTree::<Running>::new(my_tree, &app.world, entity);
	expect(out.value).to_be_some()?;
	let out = PropTree::<ActionResult>::new(my_tree, &app.world, entity);
	expect(out.children[0].value).to_be_none()?;
	expect(out.children[1].value).to_be_some()?;

	app.update();

	let out = PropTree::<ActionResult>::new(my_tree, &app.world, entity);
	expect(out.value).to_be_some()?;
	expect(out.children[0].value).to_be_none()?;
	expect(out.children[1].value).to_be_none()?;


	app.update();

	let out = PropTree::<ActionResult>::new(my_tree, &app.world, entity);
	expect(out.value).to_be_none()?;

	Ok(())
}