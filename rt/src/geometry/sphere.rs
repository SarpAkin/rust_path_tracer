use glam::Vec3;

use crate::{Ray, RayHit, geometry::Geometry};


#[derive(Debug,Clone, Copy)]
pub struct Sphere{
	pub center:Vec3,
	pub radius:f32,
}

impl Geometry for Sphere{
	fn ray_cast<'a>(ray: &'a Ray) -> Option<RayHit<'a>> {
		
		
		todo!()
	}
}