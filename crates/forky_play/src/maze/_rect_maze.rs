use bevy::prelude::Transform;
use forky_core::{graph::*, *, math::*};

use super::{_depth_first_backtrack::DepthFirstBacktrace, *};
use std::collections::HashSet;

#[derive(Debug)]
pub struct RectMaze {
	pub width: usize,
	pub height: usize,
	pub head: usize,
	pub nodes: NodeGraph,
	pub paths: NodeGraph,
}

impl DepthFirstBacktrace for RectMaze {}

impl Maze for RectMaze {
	// fn nodes(&self) -> &NodeGraph { &self.nodes }
	// fn paths_mut(&mut self) -> &mut NodeGraph { &mut self.paths }
	// fn head(&self) -> usize { self.head }

	fn shadow<'a>(&'a mut self) -> MazeShadow<'a> {
		MazeShadow {
			head: self.head,
			nodes: &self.nodes,
			paths: &mut self.paths,
		}
	}
}

impl RectMaze {
	pub fn new(width: usize, height: usize) -> RectMaze {
		let num_nodes = width * height;
		let mut nodes = NodeGraph::from_len(width * height);
		let mut paths = NodeGraph::from_len(width * height);

		for row in 0..height {
			for col in 0..width {
				let i = col + width * row;
				//left
				if col > 0 {
					nodes[i].links.insert(i - 1);
				}
				//right
				if col < width - 1 {
					nodes[i].links.insert(i + 1);
				}
				//top
				if row > 0 {
					nodes[i].links.insert(i - width);
				}
				//bottom
				if row < height - 1 {
					nodes[i].links.insert(i + width);
				}
			}
		}
		RectMaze {
			width,
			height,
			head: 0,
			nodes,
			paths,
		}
	}
	pub fn draw_maze(&self) -> Vec<u8> {
		let mut vec = self.draw_grid();
		let e_width = self.width + 1;
		let e_height = self.height + 1;


		for row in 0..self.height {
			for col in 0..self.width {
				let c1 = col + row * self.width;
				let c2 = col + row * self.width + 1;
				let c3 = c2 + self.width;
				let c4 = c1 + self.width;
				let e1 = col + row * e_width + 1;
				let e3 = e1 + e_width;
				let e2 = e3 + 1;
				let e4 = e3 - 1;
				let c1_c2_linked = self.paths.is_linked(c1, c2);
				let c2_c3_linked = self.paths.is_linked(c2, c3);
				let c3_c4_linked = self.paths.is_linked(c3, c4);
				let c4_c1_linked = self.paths.is_linked(c4, c1);
				//handle row edges
				if row == 0 && col < self.width - 1 {
					if c1_c2_linked {
						vec[e1] = u8_shape::HORIZONTAL;
					} else {
						vec[e1] = u8_shape::TOP_TEE;
					}
				}
				if row == self.height - 1 && col < self.width - 1 {
					if c1_c2_linked {
						vec[e3] = u8_shape::HORIZONTAL;
					} else {
						vec[e3] = u8_shape::BOTTOM_TEE;
					}
				}
				//handle column edges
				if col == 0 && row < self.height - 1 {
					if c4_c1_linked {
						vec[e4] = u8_shape::VERTICAL;
					} else {
						vec[e4] = u8_shape::LEFT_TEE;
					}
				}
				if col == self.width - 2 && row < self.height - 1 {
					if c2_c3_linked {
						vec[e2] = u8_shape::VERTICAL;
					} else {
						vec[e2] = u8_shape::RIGHT_TEE;
					}
				}
				//handle centers
				if row < self.height - 1 && col < self.width - 1 {
					if c1_c2_linked
						&& c2_c3_linked && c3_c4_linked
						&& c4_c1_linked
					{
						vec[e3] = u8_shape::NONE;
					} else if c1_c2_linked && c2_c3_linked && c3_c4_linked {
						vec[e3] = u8_shape::HORIZONTAL_LEFT;
					} else if c2_c3_linked && c3_c4_linked && c4_c1_linked {
						vec[e3] = u8_shape::VERTICAL_TOP;
					} else if c3_c4_linked && c4_c1_linked && c1_c2_linked {
						vec[e3] = u8_shape::HORIZONTAL_RIGHT;
					} else if c4_c1_linked && c1_c2_linked && c2_c3_linked {
						vec[e3] = u8_shape::VERTICAL_BOTTOM;
					} else if c1_c2_linked && c3_c4_linked {
						vec[e3] = u8_shape::HORIZONTAL;
					} else if c4_c1_linked && c2_c3_linked {
						vec[e3] = u8_shape::VERTICAL;
					} else if c1_c2_linked && c2_c3_linked {
						vec[e3] = u8_shape::TOP_RIGHT;
					} else if c2_c3_linked && c3_c4_linked {
						vec[e3] = u8_shape::BOTTOM_RIGHT;
					} else if c3_c4_linked && c4_c1_linked {
						vec[e3] = u8_shape::BOTTOM_LEFT;
					} else if c4_c1_linked && c1_c2_linked {
						vec[e3] = u8_shape::TOP_LEFT;
					} else if c1_c2_linked {
						vec[e3] = u8_shape::TOP_TEE;
					} else if c2_c3_linked {
						vec[e3] = u8_shape::RIGHT_TEE;
					} else if c3_c4_linked {
						vec[e3] = u8_shape::BOTTOM_TEE;
					} else if c4_c1_linked {
						vec[e3] = u8_shape::LEFT_TEE;
					} else {
						vec[e3] = u8_shape::CROSS;
					}
				}
			}
		}
		vec
	}

	pub fn draw_grid(&self) -> Vec<u8> {
		let mut buf: Vec<u8> = Vec::new();
		for row in 0..self.height + 1 {
			for col in 0..self.width + 1 {
				if row == 0 && col == 0 {
					buf.push(u8_shape::TOP_LEFT);
				} else if row == 0 && col == self.width {
					buf.push(u8_shape::TOP_RIGHT);
				} else if row == self.height && col == self.width {
					buf.push(u8_shape::BOTTOM_RIGHT);
				} else if row == self.height && col == 0 {
					buf.push(u8_shape::BOTTOM_LEFT);
				} else if row == 0 {
					buf.push(u8_shape::TOP_TEE);
				} else if row == self.height {
					buf.push(u8_shape::BOTTOM_TEE);
				} else if col == 0 {
					buf.push(u8_shape::LEFT_TEE);
				} else if col == self.width {
					buf.push(u8_shape::RIGHT_TEE);
				} else {
					buf.push(u8_shape::CROSS);
				}
			}
		}
		buf
	}
	/*
	|_|_|
	|_|_|
	c1 e1 c2
	e4    e2
	c4 e3 c3

	*/

	pub fn format(&self) -> String {
		let grid = self.draw_maze();
		let mut str = String::new();
		
		for row in 0..self.height + 1 {
			for col in 0..self.width + 1 {
				let i = col + row * (self.width + 1);
				str.push(char_shape::from_u8(grid[i]));
			}
			str.push('\n');
		}
		str
	}
	
	pub fn transforms(&self,cell_width:f32,wall_width:f32) -> Vec<(Transform, Vec<Transform>)> {
		let mut trans = Vec::new();
		let grid = self.draw_maze();

		let total_walls_w = f(self.width + 1) * cell_width;
		let total_walls_h = f(self.height + 1) * cell_width;
		
		let x_min = total_walls_w / 2. - cell_width / 2.;
		let z_min = total_walls_h / 2. - cell_width / 2.;

		for row in 0..self.height + 1 {
			for col in 0..self.width + 1 {
				let i = col + row * (self.width + 1);
				let func = mesh_shape::from_u8(grid[i]);
				let mut tt = func(cell_width,wall_width);
				let x = -x_min + cell_width * f(col);
				let z = -z_min + cell_width * f(row);
				tt.0.translation.x += x;
				tt.0.translation.z += z;
				trans.push(tt);
			}
		}
		trans
	}


	pub fn format_indices(&self) -> String {
		let mut str = String::new();

		for row in 0..self.height {
			for col in 0..self.width {
				let i = col + row * self.width;
				str.push_string(&format!("{}\t", i));
			}
			str.push('\n');
		}
		str
	}
}
