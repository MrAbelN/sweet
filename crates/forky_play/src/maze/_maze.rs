pub use forky_core::graph::*;
use bevy::render::render_graph::Node;
use forky_core::graph::NodeGraph;

pub struct MazeShadow<'a>{
	pub head:usize,
	pub nodes:&'a NodeGraph,
	pub paths:&'a mut NodeGraph
}

pub trait Maze {
	fn shadow<'a>(&'a mut self)->MazeShadow<'a>;
	// fn head(&self) -> usize;
	// fn nodes(&self) -> &NodeGraph;
	// fn paths_mut(&mut self) -> &mut NodeGraph;
}