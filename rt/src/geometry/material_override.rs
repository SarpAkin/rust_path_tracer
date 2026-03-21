use crate::{HitMaterial, Ray, RayHit, geometry::Geometry};

pub struct MaterialOverride<T: Geometry, F: Fn(RayHit) -> HitMaterial + 'static> {
	inner: T,
	func: F,
}

impl<T: Geometry, F: Fn(RayHit) -> HitMaterial + 'static> MaterialOverride<T, F> {
	pub fn new(g: T, f: F) -> MaterialOverride<T, F> { return Self { inner: g, func: f } }
}

impl<T: Geometry, F: Fn(RayHit) -> HitMaterial + 'static + Sync> Geometry for MaterialOverride<T, F> {
	fn ray_cast<'a>(&self, ray: &'a Ray) -> Option<RayHit<'a>> {
		self.inner.ray_cast(ray).map(|h| RayHit {
			ray: h.ray, //
			t: h.t,
			material: (self.func)(h),
		})
	}
}

pub trait GeometryExt: Geometry + Sized {
	fn with_material_override<F>(self, f: F) -> MaterialOverride<Self, F>
	where
		F: Fn(RayHit) -> HitMaterial + 'static,
	{
		MaterialOverride::new(self, f)
	}
}

impl<T: Geometry> GeometryExt for T {}
