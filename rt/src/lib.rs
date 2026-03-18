use glam::Vec3;

pub mod geometry;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
	origin: Vec3,
	direction: Vec3,
}

impl Ray {
	pub fn origin(&self) -> Vec3 { return self.origin; }
	pub fn direction(&self) -> Vec3 { return self.direction; }
}

pub struct HitMaterial{

}

pub struct RayHit<'a> {
	pub ray: &'a Ray,
	pub t:f32,
	pub material:HitMaterial,
}
