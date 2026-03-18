use crate::{Ray, RayHit, geometry::Geometry};

pub struct GeometryContainer<Boundary: Geometry, Children: Geometry> {
	boundary: Boundary,
	children: Vec<Children>,
}

impl<TB: Geometry, TC: Geometry> Geometry for GeometryContainer<TB, TC> {
	fn ray_cast<'a>(&self, ray: &'a Ray) -> Option<RayHit<'a>> {
		// do an early return if the ray doesn't intersect with our boundary
		if self.boundary.ray_cast(ray).is_none() {
			return None;
		}

		// ray cast into all children and return the closest one
		self.children
			.iter() //
			.map(|c| c.ray_cast(ray))
			.flatten()
			.min_by(|a, b| a.t.total_cmp(&b.t))
	}
}

impl<TB: Geometry, TC: Geometry> GeometryContainer<TB, TC> {
	pub fn new(boundary: TB,children:Vec<TC>) -> GeometryContainer<TB, TC> { Self { boundary, children } }
}
