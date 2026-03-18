use glam::Vec3;

pub mod geometry;
pub mod ray;
pub mod util;
pub use ray::Ray;



pub struct HitMaterial{
	pub normal:Vec3,
	pub albedo:Vec3,
	pub roughness:f32,
}

pub struct RayHit<'a> {
	pub ray: &'a Ray,
	pub t:f32,
	pub material:HitMaterial,
}

impl<'a> RayHit<'a> {
	pub fn hit_pos(&self) -> Vec3 {
		self.ray.origin() + self.ray.direction() * self.t
	}
}


