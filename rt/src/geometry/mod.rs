use std::any::Any;

use crate::{Ray, RayHit};

pub mod aabb;
pub mod sphere;

pub trait Geometry: Any {
	// cast a ray and returns RayHit if it hits anything. if not None is returned.
	fn ray_cast<'a>(ray: &'a Ray) -> Option<RayHit<'a>>;
}
