use std::{ops::Deref, path::Path};

use glam::{Mat4, Vec2, Vec3, Vec4};
use rt::{
	Ray,
	geometry::{Geometry, dynamic_geometry::DynGeometry, geometry_container::GeometryContainer, material_override::GeometryExt, sphere::Sphere},
};

use crate::image::{Image, encode_pixel};

mod image;

fn initialize_scene() -> impl Geometry  {
	let spheres = vec![
		Sphere::new(Vec3::new(0.0, 0.0, 10.0), 2.0, Vec3::new(0.0, 0.6, 0.5)), //
		Sphere::new(Vec3::new(5.0, 0.0, 10.0), 3.0, Vec3::new(0.5, 0.6, 0.0)), //
		Sphere::new(Vec3::new(0.0, -2.0, 16.0), 7.0, Vec3::new(0.5, 0.0, 0.5)), //
	];

	// let scene = Box::new(GeometryContainer::new())

    Sphere::bundle_spheres(spheres)
}

fn perspective_div(v: Vec4) -> Vec3 { v.truncate() / v.w }

fn make_ray_from_camera(inv_proj_view: &Mat4, screen_pos: Vec2) -> Ray {
	let a = perspective_div(inv_proj_view * Vec4::new(screen_pos.x, screen_pos.y, 0.0, 1.0));
	let b = perspective_div(inv_proj_view * Vec4::new(screen_pos.x, screen_pos.y, 1.0, 1.0));

	Ray::new(a, b - a)
}

fn render(image: &mut Image, scene: &dyn Geometry) {
	let cam_pos = Vec3::new(0.0, 0.0, 0.0);
	let cam_dir = Vec3::new(0.0, 0.0, 1.0);
	let up = Vec3::new(0.0, 1.0, 0.0);

	let aspect_ratio = image.width() as f32 / image.height() as f32;

	let view = Mat4::look_at_rh(cam_pos, cam_pos + cam_dir, up);
	let proj = Mat4::perspective_rh(90.0f32.to_radians(), aspect_ratio, 0.1, 100.0);

	let inv_proj_view = (proj * view).inverse();

	for y in 0..image.height() {
		for x in 0..image.width() {
			let screen_pos = Vec2::new(
				x as f32 / image.width() as f32, //
				y as f32 / image.height() as f32,
			) * 2.0 - 1.0;

            let ray = make_ray_from_camera(&inv_proj_view, screen_pos);
            
            if let Some(hit) = scene.ray_cast(&ray){
                let color = hit.material.albedo;
                image.set_pixel(x, y, encode_pixel(color.x, color.y, color.z, 1.0));
            }
        }
	}
}

fn main() {
	let mut image = Image::new(128, 128, encode_pixel(0.5, 0.3, 0.3, 1.0));
    let scene = initialize_scene();

    render(&mut image, &scene);

	image.write_to_png(Path::new("a.png")).unwrap();
}
