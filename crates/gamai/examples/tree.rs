#![feature(
	return_position_impl_trait_in_trait,
	associated_const_equality
)]
//this example is used for macro expansion, for usage see the `tests` directory
use gamai::*;

fn main() {
	let _ = AiPlugin::new(my_node);
	let _ = AiBundle::new(my_node);
}

fn my_bevy_system() {}
#[node_system]
fn my_node_system<N: AiNode>() {}

fn my_node() -> impl AiNode {
	gamai::tree! {
		<my_bevy_system edge=my_bevy_system>
			<my_node_system/>
		</my_bevy_system>
	}
}
