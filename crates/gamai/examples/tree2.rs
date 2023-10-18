#![feature(associated_const_equality, generic_const_exprs)]
#![allow(incomplete_features)]
use gamai::*;
//this example is for macro expansion, for usage see the `tests` directory

#[action]
fn my_system<N: AiNode>() {}

fn main() {
	let _tree0 = tree! {<MyTree/>};
	// let _tree1 = tree! {
	// 	<my_system>
	// 		<my_system>
	// 			<my_system>
	// 				<my_system/>
	// 				<my_system/>
	// 				<MyTree/>
	// 			</my_system>
	// 		</my_system>
	// 	</my_system>
	// };
}

#[tree_builder]
pub fn MyTree() -> impl AiNode {
	tree! {
		<my_system>
			<my_system/>
		</my_system>
	}
}