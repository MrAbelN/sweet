use super::*;
use crate::bezier3;
use anyhow::{anyhow, Result};
use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LinearSpline {
	pub p0: Vec3,
	pub p1: Vec3,
}

impl LinearSpline {
	pub fn new(p0: Vec3, p1: Vec3) -> Self { Self { p0, p1 } }
}

impl SplineType for LinearSpline {
	fn get_points(&self) -> Vec<Vec3> { vec![self.p0, self.p1] }

	fn set_point(&mut self, pos: Vec3, index: u32) -> Result<()> {
		match index {
			0 => {
				self.p0 = pos;
				Ok(())
			}
			1 => {
				self.p1 = pos;
				Ok(())
			}
			_ => Err(anyhow!("Invalid index")),
		}
	}

	fn set_first(&mut self, pos: Vec3) { self.p0 = pos }

	fn set_last(&mut self, pos: Vec3) { self.p1 = pos }

	fn position(&self, t: f32) -> Vec3 {
		//
		bezier3::linear(self.p0, self.p1, t)
	}

	fn tangent(&self, _t: f32) -> Vec3 {
		bezier3::tangent_linear(self.p0, self.p1)
	}
}
