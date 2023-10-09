use crate::*;
use bevy_ecs::prelude::*;
use bevy_ecs::query::WorldQuery;
use std::marker::PhantomData;


/// An AiNode is a node and edge system, and a set of child nodes.
pub trait AiNode: 'static + Send + Sync + Sized + IntoNodeId {
	// we need to repeat the consts for implementations as <Self::ID> is not allowed
	// const GRAPH_ID: usize = <Self as IntoNodeId>::GRAPH_ID;
	// const GRAPH_DEPTH: usize = <Self as IntoNodeId>::GRAPH_DEPTH;
	// const CHILD_INDEX: usize = <Self as IntoNodeId>::CHILD_INDEX;
	// const PARENT_DEPTH: usize = <Self as IntoNodeId>::PARENT_DEPTH;
	/// Tuple Query used to access child states: `(Entity,(Child1,(Child2)))`
	type ChildQuery: WorldQuery;
	type ChildBundle: 'static + Send + Sync + Default + Bundle;

	type Query<'w, 's> = Query<'w, 's, Self::ChildQuery>;
	fn entity<'a>(item: &<Self::ChildQuery as WorldQuery>::Item<'a>) -> Entity;
	fn children<'a>(
		item: <Self::ChildQuery as WorldQuery>::Item<'a>,
	) -> Vec<ChildState<'a>>;
	fn add_systems(self, schedule: &mut Schedule);

	fn get_child(&self, index: usize) -> &dyn NodeInspector;
	fn get_child_owned(self, index: usize) -> Box<dyn NodeInspector>;
}


#[derive(Debug, Default, Clone, Component)]
pub struct PhantomComponent<T>(pub PhantomData<T>);

impl<T> PhantomComponent<T> {
	pub fn new() -> Self { Self(PhantomData) }
}


pub trait NodeInspector {
	fn node_state(&self, world: &World, entity: Entity) -> Option<NodeState>;
	fn edge_state(&self, world: &World, entity: Entity) -> Option<EdgeState>;
	fn child(&self, index: usize) -> &dyn NodeInspector;
	fn child_owned(self, index: usize) -> Box<dyn NodeInspector>;
	fn graph_id(&self) -> usize;
	fn child_index(&self) -> usize;
	fn graph_depth(&self) -> usize;
	fn parent_depth(&self) -> usize;
}

impl<T: AiNode + Sized> NodeInspector for T {
	fn graph_id(&self) -> usize { Self::GRAPH_ID }
	fn child_index(&self) -> usize { Self::CHILD_INDEX }
	fn graph_depth(&self) -> usize { Self::GRAPH_DEPTH }
	fn parent_depth(&self) -> usize { Self::PARENT_DEPTH }
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