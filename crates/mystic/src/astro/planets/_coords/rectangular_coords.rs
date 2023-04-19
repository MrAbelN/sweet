use derive_deref::{Deref, DerefMut};

#[derive(Debug, Copy, Clone, PartialEq, Deref, DerefMut)]
pub struct GeoCoords(pub RectangluarCoords);
#[derive(Debug, Copy, Clone, PartialEq, Deref, DerefMut)]
pub struct HelioCoords(pub RectangluarCoords);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RectangluarCoords {
	//right
	pub x: f64,
	//forward
	pub y: f64,
	//up
	pub z: f64,
}

impl RectangluarCoords {
	pub fn new(x: f64, y: f64, z: f64) -> Self { Self { x, y, z } }
	pub fn length(&self) -> f64 {
		f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
	}
	pub fn length_xy(&self) -> f64 {
		f64::sqrt(self.x * self.x + self.y * self.y)
	}

	pub fn flat_angle(&self) -> f64 { f64::atan2(self.y, self.x) }
	pub fn up_angle(&self) -> f64 { f64::atan2(self.z, self.length_xy()) }
}
impl std::ops::Add for RectangluarCoords {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self {
			x: self.x + other.x,
			y: self.y + other.y,
			z: self.z + other.z,
		}
	}
}

impl std::ops::Sub for RectangluarCoords {
	type Output = Self;

	fn sub(self, other: Self) -> Self {
		Self {
			x: self.x - other.x,
			y: self.y - other.y,
			z: self.z - other.z,
		}
	}
}
