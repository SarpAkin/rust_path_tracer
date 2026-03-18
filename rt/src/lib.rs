use glam::Vec3;

pub mod geometry;
pub mod ray;
pub mod util;
pub use ray::Ray;



pub struct HitMaterial{
	pub normal:Vec3,
	pub albedo:Vec3,
}

pub struct RayHit<'a> {
	pub ray: &'a Ray,
	pub t:f32,
	pub material:HitMaterial,
}
