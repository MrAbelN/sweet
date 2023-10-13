use crate::*;
use bevy_app::Plugin;
use bevy_ecs::prelude::*;
use bevy_ecs::query::WorldQuery;
use std::marker::PhantomData;


/// An AiNode is a node and edge system, and a set of child nodes.
// pub trait AiNode {
pub trait AiNode: 'static + Send + Sync + TreePath {
	/// Tuple Query used to access child states: `(Entity,(Child1,(Child2)))`
	type ChildQuery: WorldQuery;
	type ChildBundle: 'static + Send + Sync + Default + Bundle;

	type Query<'w, 's> = Query<'w, 's, Self::ChildQuery>;
	fn entity<'a>(item: &<Self::ChildQuery as WorldQuery>::Item<'a>) -> Entity;
	fn children<'a>(
		item: <Self::ChildQuery as WorldQuery>::Item<'a>,
	) -> Vec<ChildState<'a>>;
	fn add_systems(self, schedule: &mut Schedule);

	fn get<'a, Component: NodeComponent>(
		&'a self,
		world: &'a World,
		entity: Entity,
	) -> Option<&'a Component::Value<Self>> {
		Component::get(self, world, entity)
	}
	fn get_child(&self, index: usize) -> &dyn NodeInspector;
	fn get_child_owned(self, index: usize) -> Box<dyn NodeInspector>;
	fn get_children(&self) -> Vec<&dyn NodeInspector>;

	/// Copies self, with a different path.
	fn into_child<Path: TreePath>(self) -> impl AiNode;
	/// Fixes paths of all children to be relative to self.
	fn into_root(self) -> impl AiNode { self.into_child::<Self>() }

	fn bundle(self) -> impl Bundle { AiBundle::new(self) }
	fn bundle_inactive(self) -> impl Bundle { AiBundle::inactive(self) }
	fn plugin(self) -> impl Plugin { AiPlugin::new(self) }
}

#[derive(Debug, Default, Clone, Component)]
pub struct PhantomComponent<T>(pub PhantomData<T>);

impl<T> PhantomComponent<T> {
	pub fn new() -> Self { Self(PhantomData) }
}

/// Base type for nodes, must be Send/Sync because used in components for distinguishing nodes
pub trait NodeInspector: 'static + Send + Sync {
	fn node_state(&self, world: &World, entity: Entity) -> Option<NodeState>;
	fn edge_state(&self, world: &World, entity: Entity) -> Option<EdgeState>;
	fn child(&self, index: usize) -> &dyn NodeInspector;
	fn child_owned(self, index: usize) -> Box<dyn NodeInspector>;
	fn graph_id(&self) -> usize;
	fn child_index(&self) -> usize;
	fn graph_depth(&self) -> usize;
}

impl<T: AiNode + Sized> NodeInspector for T {
	fn graph_id(&self) -> usize { Self::GRAPH_ID }
	fn child_index(&self) -> usize { Self::CHILD_INDEX }
	fn graph_depth(&self) -> usize { Self::DEPTH }
	fn node_state(&self, world: &World, entity: Entity) -> Option<NodeState> {
		world
			.entity(entity)
			.get::<DerefNodeState<Self>>()
			.map(|state| **state)
	}

	fn edge_state(&self, world: &World, entity: Entity) -> Option<EdgeState> {
		world
			.entity(entity)
			.get::<DerefEdgeState<Self>>()
			.map(|state| **state)
	}

	fn child(&self, index: usize) -> &dyn NodeInspector {
		self.get_child(index)
	}
	fn child_owned(self, index: usize) -> Box<dyn NodeInspector> {
		self.get_child_owned(index)
	}
}
