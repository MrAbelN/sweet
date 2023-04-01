use super::*;
use anyhow::Result;
use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum Spline {
	Linear(LinearSpline),
	Quadratic(QuadraticSpline),
	Cubic(CubicSpline),
}


impl SplineType for Spline {
	fn get_points(&self) -> Vec<Vec3> {
		match self {
			Spline::Linear(spline) => spline.get_points(),
			Spline::Quadratic(spline) => spline.get_points(),
			Spline::Cubic(spline) => spline.get_points(),
		}
	}
	fn set_point(&mut self, pos: Vec3, index: u32) -> Result<()> {
		match self {
			Spline::Linear(spline) => spline.set_point(pos, index),
			Spline::Quadratic(spline) => spline.set_point(pos, index),
			Spline::Cubic(spline) => spline.set_point(pos, index),
		}
	}
	fn set_first(&mut self, p: Vec3) {
		match self {
			Spline::Linear(spline) => spline.set_first(p),
			Spline::Quadratic(spline) => spline.set_first(p),
			Spline::Cubic(spline) => spline.set_first(p),
		}
	}
	fn set_last(&mut self, p: Vec3) {
		match self {
			Spline::Linear(spline) => spline.set_last(p),
			Spline::Quadratic(spline) => spline.set_last(p),
			Spline::Cubic(spline) => spline.set_last(p),
		}
	}
	fn position(&self, t: f32) -> Vec3 {
		match self {
			Spline::Linear(spline) => spline.position(t),
			Spline::Quadratic(spline) => spline.position(t),
			Spline::Cubic(spline) => spline.position(t),
		}
	}

	fn tangent(&self, t: f32) -> Vec3 {
		match self {
			Spline::Linear(spline) => spline.tangent(t),
			Spline::Quadratic(spline) => spline.tangent(t),
			Spline::Cubic(spline) => spline.tangent(t),
		}
	}
}
