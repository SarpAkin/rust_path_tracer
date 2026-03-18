use crate::geometry::Geometry;

pub struct DynGeometry {
	inner: Box<dyn Geometry>,
}

impl DynGeometry {
	pub fn new<T: Geometry>(g: T) -> DynGeometry { DynGeometry { inner: Box::new(g) } }
	pub fn new_from_boxed<T: Geometry>(g: Box<T>) -> DynGeometry { DynGeometry { inner: g } }
}

impl Geometry for DynGeometry {
	fn ray_cast<'a>(&self, ray: &'a crate::Ray) -> Option<crate::RayHit<'a>> { self.inner.ray_cast(ray) }
}

pub trait IntoDynGeometry: Geometry + Sized {
	fn into_dyn_geometry(self) -> DynGeometry { DynGeometry::new(self) }
	fn into_dyn_geometry_boxed(self: Box<Self>) -> DynGeometry { DynGeometry::new_from_boxed(self) }
}

impl<T: Geometry> IntoDynGeometry for T {}
