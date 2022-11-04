use crate::random;
use extend::ext;
use std::{collections::HashSet, fmt::Display, hash::Hash};


pub type NodeGraph = Vec<Node>;

#[ext]
pub impl NodeGraph {
	fn clear_links(&mut self) {
		for node in self.iter_mut() {
			node.links.clear();
		}
	}
	fn is_linked(&self, a: usize, b: usize) -> bool {
		if a >= self.len() || b >= self.len() {
			return false;
		}
		self[a].links.contains(&b)
	}


	fn link_randomly(&mut self) {
		let iter = self.iter().enumerate();
		let mut to_link: Vec<(usize, usize)> = Vec::new();
		for (i, node) in self.iter().enumerate() {
			for neighbor in node.links.iter() {
				let num = random::percent_i32();
				if num < 20 {
					to_link.push((i, neighbor.clone()));
					// self.link(i, neighbor.clone());
				}
			}
		}
		to_link.iter().for_each(|(a, b)| self.link(*a, *b));
		// self.nodes[a].links.insert(b);
		// self.nodes[b].links.insert(a);
	}


	fn link(&mut self, a: usize, b: usize) {
		self[a].links.insert(b);
		self[b].links.insert(a);
	}
	fn unlink(&mut self, a: usize, b: usize) {
		self[a].links.remove(&b);
		self[b].links.remove(&a);
	}

	fn next_unvisited(&self, i: usize, visited: &HashSet<usize>) -> Option<&usize> {
		self[i]
			// ._get(i)
			.links
			.iter()
			.filter(|n| !visited.contains(n))
			.next()
	}
}


#[derive(Clone, Debug, Default)]
pub struct Node {
	// id:usize,
	pub links: HashSet<usize>,
}

impl Node {
	pub fn new() -> Node {
		Node {
			// id,
			links: HashSet::new(),
		}
	}
}
