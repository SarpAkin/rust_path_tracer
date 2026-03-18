use core::f32;

use glam::Vec3;

use crate::{HitMaterial, Ray, RayHit, geometry::Geometry};

pub struct AlwaysHit;

impl Geometry for AlwaysHit {
	fn ray_cast<'a>(&self, ray: &'a Ray) -> Option<RayHit<'a>> {
		return Some(RayHit {
			ray,
			t: f32::INFINITY,
			material: HitMaterial { normal: Vec3::ZERO, albedo: Vec3::ZERO,roughness:0.0 },
		});
	}
}
