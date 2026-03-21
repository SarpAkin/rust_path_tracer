use glam::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
	origin: Vec3,
	direction: Vec3,
}

impl Ray {
	pub fn origin(&self) -> Vec3 { self.origin }
	pub fn direction(&self) -> Vec3 { self.direction }
	pub fn direction_reciprocal(&self) -> Vec3 { 1.0 / self.direction }
	pub fn new(ro: Vec3, rd: Vec3) -> Ray {
		Self {
			origin: ro,
			// ensure that direction is normalized
			direction: rd.normalize(),
		}
	}
}
