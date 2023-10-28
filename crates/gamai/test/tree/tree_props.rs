use bevy_ecs::prelude::*;
use gamai::common_actions::*;
use gamai::common_selectors::*;
use gamai::*;
use sweet::*;

#[sweet_test]
pub fn it_works() -> Result<()> {
	let my_tree = || {
		tree! {
			// <sequence props=[true,3usize]>
			<sequence props=(true,3usize)>
				<node_always_succeed/>
			</sequence>
		}
	};

	let mut world = World::new();
	// wr
	let entity = world.spawn(TreeBundle::inactive(my_tree)).id();

	expect(world.all_component_ids(entity).count()).to_be(2)?;

	// let components = all_component_infos(&world, entity)
	// 	.map(|info| info.name())
	// 	.collect::<Vec<_>>()
	// 	.join("\n\n");
	// println!("{components}");

	Ok(())
}

// pub fn all_components_to_string(world: &World, entity: Entity) -> String {
// 	if let Some(components) = get_all_components(world, entity) {
// 		let mut out = String::new();
// 		for component in components {
// 			if let Some(info) = components.get_info(id)
// 			out.push_str(&format!("{:?}\n", component));
// 		}
// 		out
// 	} else {
// 		"None".to_string()
// 	}
// }