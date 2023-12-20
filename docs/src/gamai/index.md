<div style="display:flex; justify-content:space-between">
<h1>Gamai</h1>
<img src="images/spotted-gum.jpg" height=200px>
</div>
<br/>

> *Very early stage warning:*
> - breaking changes on patch versions
> - continued development not guaranteed
> - outdated docs
> - bugs ahoy

Gamai is a modular task switching library suitable for game AI, robotics & other performance-critical environments. It is built with `bevy_ecs` which allows for parallelism and no blackboard. The primitives it provies can be used for multiple selector paradigms like Behaviour Trees, State Machines and Utility AI.

## Features

- 🐦 Powered by `bevy_ecs` and `petgraph`
- 🔥 Highly Parallel
- ✍️ No Blackboard
- 🌈 Multi-paradigm
- 🌍 With or without Bevy Engine
- 🌴 Create/edit graphs at runtime
- 🧩 Multiple graphs per entity

## Overview

This is my third attempt at a modular AI architecture for ECS, the previous two attempts went the way of the dodo:
1. Shoehorn non-ecs solutions into bevy, which sucked mostly because of blackboards. 
2. Get clever with generics and create distinct types *per node* of a graph. This allowed for an entire graph to be stored as components on a single entity but was not great for a bunch of reasons. The dealbreaker was not being able to create/edit graphs at runtime.

I'm quite confident in this third approach, representing graphs as linked entities. 

### Actions

Actions without children usually execute some behavior then return a `RunResult::Success` or a `RunResult::Failure`.

An `action` is a bevy component struct with an associated system. Currently all actions must implement `Clone`, `Component`, `serde::Serialize` and `serde::Deserialize`.

```rust
#[action(system=print_action)]
#[derive(Clone, Component, Serialize, Deserialize)]
pub struct PrintAction(pub value: String);

fn print_action(mut commands: Commands, query: Query<(Entity,&PrintAction), With<Running>){
	for (entity, action) in query.iter(){
		println!("Print Action: {}", action.0);
		commands.entity(entity).insert(RunResult::Success);
	}
}
```

### Shared Actions
To solve the problem of, say every scoring system wanting a `Query<&mut Score>` which would break parallelism, each action has a distinct component, and fields marked as `#[shared]` will be copied at the end of each tick when they change.

The below action will add a `Score` component to this entity and update it whenever the `ScoringAction` changes.

```rust
#[action(system=scoring_action)]
pub struct ScoringAction{
	#[shared]
	pub score: Score
};
```

Now we can have a utility selector that immutably queries the `Score` component of its children, allowing for full parallelism.

### Next Steps

Documentation is WIP, in the meantime have a look at `./crates/gamai/test/selectors` for examples of selectors and how they are used.

### Name
`Gamai` is a library based on trees and is named after one of my favorites, the [Australian Spotted Gum](https://en.wikipedia.org/wiki/Corymbia_maculata). It also sounds like `game ai`.