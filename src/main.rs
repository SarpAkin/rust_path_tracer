use std::{ops::Deref, path::Path};

use glam::{IVec3, Mat4, Vec2, Vec2Swizzles, Vec3, Vec3Swizzles, Vec4};
use rand::{Rng, RngExt};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rt::{
	HitMaterial, Ray,
	geometry::{
		Geometry,
		aabb::AABB,
		dynamic_geometry::{DynGeometry, IntoDynGeometry},
		geometry_container::{GeometryContainer, GeometryPack},
		material_override::GeometryExt,
		sphere::Sphere,
	},
};

use crate::image::{Image, encode_pixel};

mod image;

fn make_checker_board() -> impl Geometry {
	AABB::new(Vec3::new(-100.0, -1.0, -100.0), Vec3::new(100.0, 0.0, 100.0), Vec3::ONE, 0.5)
		.with_material_override(|h| {
			let pos = h.hit_pos();
			let n = (pos.floor().as_ivec3()).element_sum() & 1;
			let color = if n == 1 { Vec3::new(0.8, 0.8, 0.85) } else { Vec3::new(0.12, 0.12, 0.15) };
			HitMaterial { normal: h.material.normal, albedo: color, roughness: h.material.roughness }
		})
}

fn encode2(n: Vec3) -> Vec2 {
	let scale = 1.7777;
	let mut enc = n.xy() / (n.z + 1.0);
	enc /= scale;
	enc = enc * 0.5 + 0.5;
	return enc;
}

fn encode_spherical(n: Vec3) -> Vec2 {
	let p = (n.z * 8.0 + 8.0).sqrt();
	n.xy() / p + 0.5
}

fn oct_wrap(v: Vec2) -> Vec2 { (1.0 - (v.yx()).abs()) * v.xy().signum() }

fn encode_octa(mut n: Vec3) -> Vec2 {
	n /= (n.x).abs() + (n.y).abs() + (n.z).abs();
	let mut xy = if n.z >= 0.0 { n.xy() } else { oct_wrap(n.xy()) };
	xy = xy * 0.5 + 0.5;
	return xy;
}

fn checkerboard_sphere() -> impl Geometry {
	//
	Sphere::new(Vec3::new(0.0, 5.0, 0.0), 2.0, Vec3::ZERO, 0.0).with_material_override(|h| {
		let spherical = encode_octa(h.material.normal);
		let tile_pos = (spherical * 25.0).as_ivec2();

		// let color = Vec3::new(spherical.x, spherical.y, 0.0);
		let checker_board = if tile_pos.element_sum() & 1 == 0 { 0.9 } else { 0.1 };
		let color = Vec3::new(spherical.x, spherical.y, checker_board);

		HitMaterial { normal: h.material.normal, albedo: color, roughness: 0.7 }
	})
}

fn initialize_scene() -> impl Geometry {
	let spheres = vec![
		//
		Sphere::new(Vec3::new(0.0, 6.0, 0.0), 2.5, Vec3::ONE, 0.01),
		Sphere::new(Vec3::new(3.0, 1.5, 0.0), 0.9, Vec3::new(0.5, 0.5, 1.0), 0.1),
		// Sphere::new(Vec3::new(0.0, 6.0, 0.0), 2.5, Vec3::ONE, 0.01),
	]
	.into_geometry_container()
	.into_dyn_geometry();

	// Sphere::bundle_spheres(spheres)

	let boxes = AABB::bundle(vec![
		// AABB::new(Vec3::new(-10.0, -2.0, -10.0),Vec3::new(10.0,0.0,10.0),Vec3::ONE,0.5),
		AABB::new(Vec3::new(-2.0, -2.0, -2.0), Vec3::new(2.0, 2.0, 2.0), Vec3::new(0.5, 0.5, 0.5), 0.01),
	])
	.into_dyn_geometry();

	let checkerboard = make_checker_board().into_dyn_geometry();

	vec![
		// checkerboard_sphere().into_dyn_geometry(),
		checkerboard,
		spheres,
	]
	.into_geometry_container()
}

fn perspective_div(v: Vec4) -> Vec3 { v.truncate() / v.w }

fn make_ray_from_camera(inv_proj_view: &Mat4, screen_pos: Vec2) -> Ray {
	let a = perspective_div(inv_proj_view * Vec4::new(screen_pos.x, screen_pos.y, 0.0, 1.0));
	let b = perspective_div(inv_proj_view * Vec4::new(screen_pos.x, screen_pos.y, 1.0, 1.0));

	Ray::new(a, b - a)
}

fn calculate_skybox_color(rd: Vec3) -> Vec3 {
	let sun_dir = Vec3::new(0.1, -0.9, 0.1).normalize();
	let sun_color = Vec3::ONE * 2.5;
	(-rd.dot(sun_dir)).max(0.0) * sun_color
}

fn rand_unit_vector() -> Vec3 {
	let mut rand = rand::rng();

	Vec3::new(
		rand.random_range(-1.0f32..1.0), //
		rand.random_range(-1.0f32..1.0), //
		rand.random_range(-1.0f32..1.0), //
	)
	.normalize()
}

fn cast_ray_and_calculate_color(ray: Ray, scene: &dyn Geometry, recursion_limit: i32) -> Vec3 {
	let Some(hit) = scene.ray_cast(&ray) else {
		return calculate_skybox_color(ray.direction());
	};

	let hit_pos = hit.hit_pos();
	let hit_normal = hit.material.normal;
	let hit_roughness = hit.material.roughness;

	// hit_normal.lerp(rhs, s)

	if recursion_limit <= 0 {
		return hit.material.albedo;
	}

	let num_rays = if recursion_limit > 1 { 20 } else { 5 };
	let reflect_color_sum = (0..num_rays)
		.map(|_| {
			let normal = hit_normal.lerp(rand_unit_vector(), hit_roughness).normalize();
			let dir = ray.direction().reflect(normal);
			let ray1 = Ray::new(hit_pos + dir * 0.001, dir);
			cast_ray_and_calculate_color(ray1, scene, recursion_limit - 1)
		})
		.sum::<Vec3>()
		/ num_rays as f32;

	hit.material.albedo * reflect_color_sum
}

fn sample(inv_proj_view: &Mat4, scene: &dyn Geometry, screen_pos: Vec2) -> Vec3 {
	let ray = make_ray_from_camera(&inv_proj_view, screen_pos);
	let color = cast_ray_and_calculate_color(ray, scene, 2);
	color
}

fn render(scene: &dyn Geometry) -> Image {
	let cam_pos = Vec3::new(-10.0, 10.0, -10.0);
	let cam_dir = Vec3::new(1.0, 0.0, 1.0);
	let up = Vec3::new(0.0, 1.0, 0.0);

	let dims = 2048;

	let aspect_ratio = 1.0;

	let view = Mat4::look_at_rh(cam_pos, cam_pos + cam_dir, up);
	let proj = Mat4::perspective_rh(90.0f32.to_radians(), aspect_ratio, 0.1, 100.0);

	let inv_proj_view = (proj * view).inverse();

	let pixel_size = Vec2::splat(1.0 / dims as f32);

	let super_sample_offsets = [
		Vec2::new(0.50, 0.50), //
		Vec2::new(0.50, -0.50),
		Vec2::new(-0.50, 0.50),
		Vec2::new(-0.50, -0.50),
	];

	let vec: Vec<u32> = (0..dims)
		.into_par_iter()
		.flat_map_iter(|y| {
			(0..dims).map(move |x| {
				let mut screen_pos = Vec2::new(
					x as f32 / dims as f32, //
					y as f32 / dims as f32,
				) * 2.0 - 1.0;

				screen_pos.y = -screen_pos.y;

				let color = super_sample_offsets
					.iter() //
					.map(|o| sample(&inv_proj_view, scene, screen_pos + (o * pixel_size)))
					.sum::<Vec3>() / super_sample_offsets.len() as f32;
				encode_pixel(color.x, color.y, color.z, 1.0)
			})
		})
		.collect();

	Image::from_vec(dims, dims, vec)
}

fn main() {
	let scene = initialize_scene();

	let image = render(&scene);

	image.write_to_png(Path::new("output.png")).unwrap();
}
