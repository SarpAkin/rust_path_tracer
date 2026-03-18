use glam::Vec3;

use crate::{
	HitMaterial, Ray, RayHit,
	geometry::{AlwaysHit, DynGeometry, Geometry, GeometryContainer},
	util::solve_quadratic,
};

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
	pub center: Vec3,
	pub radius: f32,
	pub color: Vec3,
}

impl Sphere {
	pub fn new(c: Vec3, r: f32, color: Vec3) -> Sphere { Self { center: c, radius: r, color } }

	pub fn bundle_spheres(spheres: Vec<Sphere>) -> impl Geometry {
		DynGeometry::new(GeometryContainer::new(AlwaysHit, spheres))
	}
}

impl Geometry for Sphere {
	fn ray_cast<'a>(&self, ray: &'a Ray) -> Option<RayHit<'a>> {
		let delta = ray.origin() - self.center;
		let delta_cos_dir = delta.dot(ray.direction());

		let roots = solve_quadratic(
			1.0,
			2.0 * delta_cos_dir, //
			delta.length_squared() - self.radius * self.radius,
		)?;
		let t = roots[0];
		if t < 0.0 {
			return None;
		}

		let normal = (delta + t * ray.direction()).normalize();

		Some(RayHit {
			ray,
			t,
			material: HitMaterial {
				albedo: self.color, //
				normal,
				roughness: 0.3,
			},
		})
	}
}
