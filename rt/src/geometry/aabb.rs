use std::ops::Index;

use glam::Vec3;

use crate::{
	HitMaterial, Ray, RayHit,
	geometry::{AlwaysHit, Geometry, GeometryContainer},
};

pub struct AABB {
	pub center: Vec3,
	pub half_size: Vec3,
	pub color: Vec3,
	pub roughness: f32,
}

impl AABB {
	pub fn new(min: Vec3, max: Vec3, color: Vec3, roughness: f32) -> AABB {
		let mid = (min + max) * 0.5;
		let ext = (max - min).abs();
		AABB { center: mid, half_size: ext, color, roughness }
	}

	pub fn min(&self) -> Vec3 { self.center - self.half_size }
	pub fn max(&self) -> Vec3 { self.center + self.half_size }

	pub fn bundle(aabbs: Vec<AABB>) -> impl Geometry {
		let min = aabbs.iter().map(|a| a.min()).fold(Vec3::INFINITY, |a, b| a.min(b));
		let max = aabbs.iter().map(|a| a.max()).fold(Vec3::NEG_INFINITY, |a, b| a.max(b));

		GeometryContainer::new(AlwaysHit, aabbs)
	}
}

fn max_element_and_index(v: Vec3) -> (f32, i32) {
	if v.x > v.y {
		if v.x > v.z { (v.x, 0) } else { (v.z, 2) }
	} else {
		if v.y > v.z { (v.y, 1) } else { (v.z, 2) }
	}
}

impl Geometry for AABB {
	fn ray_cast<'a>(&self, ray: &'a Ray) -> Option<RayHit<'a>> {
		let rcp = ray.direction_reciprocal();
		let delta = self.center - ray.origin();

		let dim_diff_t = (self.half_size * rcp).abs();
		let mid_t = delta * rcp;
		let near_t = mid_t - dim_diff_t;
		let far_t = mid_t + dim_diff_t;

		let (near_max, axis) = max_element_and_index(near_t);
		if near_max < 0.0 {
			return None;
		}

		let far_min = far_t.min_element();

		if near_max > far_min {
			return None;
		}

		let mut normal = Vec3::ZERO;
		normal[axis as usize] = -ray.direction()[axis as usize].signum();

		Some(RayHit {
			ray, //
			t: near_max,
			material: HitMaterial { normal: normal, albedo: self.color, roughness: self.roughness },
		})
	}
}
