use std::any::Any;

use crate::{Ray, RayHit};

pub mod aabb;
pub mod sphere;
pub mod geometry_container;
pub mod dynamic_geometry;
pub mod always_hit;
pub mod material_override;

pub use sphere::Sphere;
pub use geometry_container::GeometryContainer;
pub use dynamic_geometry::DynGeometry;
pub use always_hit::AlwaysHit;
pub use material_override::MaterialOverride;

pub trait Geometry: Any {
	// cast a ray and returns RayHit if it hits anything. if not None is returned.
	fn ray_cast<'a>(&self,ray: &'a Ray) -> Option<RayHit<'a>>;
}


