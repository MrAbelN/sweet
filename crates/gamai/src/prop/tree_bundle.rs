use super::*;
use crate::*;
use bevy_ecs::bundle::Bundle;

/// A bundle that adds props to all nodes in a tree.
pub struct TreeBundle;

impl TreeBundle {
	/// Create a bundle with the 
	/// [Components](bevy_ecs::component::Component) 
	/// and [Props](Prop) specified in this tree.
	pub fn new<M, T: IntoProp + Clone, Node: AiNode>(
		node: impl IntoNode<M, Out = Node>,
	) -> impl Bundle {
		node.into_node().into_bundle()
	}

	/// Recursively create a prop with the given value for each node in the tree.
	pub fn recursive<M, T: IntoProp + Clone, Node: AiNode>(
		_node: impl IntoNode<M, Out = Node>,
		value: T,
	) -> Node::BundleRecursive<T> {
		Node::tree_bundle(value)
	}
	/// Create a prop with the given value for the root node only.
	pub fn root<M, T: IntoProp + Default + Clone, Node: AiNode>(
		_node: impl IntoNode<M, Out = Node>,
		value: T,
	) -> Prop<T, Node> {
		Prop::new(value)
	}
}
