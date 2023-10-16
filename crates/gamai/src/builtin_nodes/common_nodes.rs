// pub mod common_solvers {
use crate::*;
use bevy_ecs::prelude::*;


// should probably be a node not a node system
// pub fn empty_node() {}

//TODO deprecate, dont need it, instead empty attribute
#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
#[allow(non_camel_case_types)]
pub struct empty_node;

impl IntoAction for empty_node {
	const IS_EMPTY: bool = true;
	fn into_action_configs<Node: AiNode>(
		self,
	) -> bevy_ecs::schedule::SystemConfigs {
		(|| {}).into_configs()
	}
}

#[action]
pub fn first_passing_score<N: AiNode>(
	mut commands: Commands,
	mut query: Query<
		(N::ChildQuery<Score>, N::ChildQueryOptMut<NodeState>),
		With<Prop<NodeState, N>>,
	>,
) {
	for (scores, states) in query.iter_mut() {
		let mut children = N::children(scores)
			.into_iter()
			.zip(N::children_opt_mut(states).into_iter())
			.collect::<Vec<_>>();
		for (score, state) in children.iter_mut() {
			if **score.get() != Score::Fail {
				state.set(&mut commands, Some(NodeState::Running));
			}
		}
	}
}

//TODO handle failure
#[action]
pub fn parallel<N: AiNode>(
	mut _commands: Commands,
	mut _query: Query<
		(N::ChildQueryMut<Score>, N::ChildQueryOptMut<NodeState>),
		With<Prop<NodeState, N>>,
	>,
) {
	// for children in query.iter_mut() {
	// 	let mut children = N::children_mut(children);
	// 	for child in children.iter_mut() {
	// 		child.set_node_state(&mut commands, Some(NodeState::Running));
	// 	}
	// }
}

#[action]
pub fn score_always_pass<N: AiNode>(mut query: Query<&mut ScoreProp<N>>) {
	for mut score in query.iter_mut() {
		**score = Score::Pass;
	}
}
#[action]
pub fn score_always_fail<N: AiNode>(mut query: Query<&mut ScoreProp<N>>) {
	for mut score in query.iter_mut() {
		**score = Score::Fail;
	}
}
#[action]
pub fn node_always_succeed<N: AiNode>(
	mut query: Query<&mut Prop<NodeState, N>>,
) {
	for mut node in query.iter_mut() {
		**node = NodeState::Success;
	}
}
#[action]
pub fn node_always_fail<N: AiNode>(mut query: Query<&mut Prop<NodeState, N>>) {
	for mut node in query.iter_mut() {
		**node = NodeState::Failure;
	}
}
#[action]
pub fn print_on_run<N: AiNode>(mut query: Query<&mut Prop<NodeState, N>>) {
	// println!("print_on_run: running..");
	for mut node in query.iter_mut() {
		println!("Action: Running {:?}", **node);
		**node = NodeState::Success;
	}
}

pub fn cleanup_state<N: AiNode>(
	mut commands: Commands,
	query: Query<(Entity, &Prop<NodeState, N>), Changed<Prop<NodeState, N>>>,
) {
	for (entity, node) in query.iter() {
		if **node != NodeState::Running {
			commands.entity(entity).remove::<Prop<NodeState, N>>();
		}
	}
}
// pub fn cleanup_child_state<N: AiNode>(
// 	mut commands: Commands,
// 	mut query: Query<ChildIter<NodeState, N>>,
// ) {
// 	for children in query.iter_mut() {
// 		for mut child in N::children(children) {
// 			if let Some(val) = &child.value && ***val != NodeState::Running {
// 				child.set_node_state(&mut commands, None);
// 			}
// 		}
// 	}
// }
