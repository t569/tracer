use std::fs::File;
use std::io::Write;

pub mod core;
pub mod color;
pub mod ray;

pub use core::Vec3;
pub use color::write_color;
pub use ray::Ray;

pub fn write_to_file(filename: &str, data: &str) {
    let mut file = File::create(filename).expect("Unable to create file");
    file.write_all(data.as_bytes()).expect("Unable to write data");
}

pub fn ray_color(ray: &Ray) -> Vec3<f64> {
    // Simple ray color function that returns a gradient based on the ray's direction
    let centre = Vec3::new(0.0, 0.0, -1.0);

    if hit_sphere(&centre, 0.5, ray){
        return Vec3::new(1.0, 0.0, 0.0); // Red color if the ray hits the sphere
    }

    let unit_direction = ray.direction().normalize();

    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3::new(
        (1.0 - t) * 1.0 + t * 0.5, // Interpolating between white and light blue
        (1.0 - t) * 1.0 + t * 0.7, // Interpolating between white and light blue
        (1.0 - t) * 1.0 + t * 1.0, // Interpolating between white and light blue
    )
}


pub fn hit_sphere(center: &Vec3<f64>, radius: f64, ray: &Ray) -> bool {
    // nahhhh, imma do it my own way

    let oc = center - ray.origin();
    let unit_ray_direction = ray.direction().normalize();

    let a = unit_ray_direction.dot(&unit_ray_direction);
    let b = -2.0 * oc.dot(&unit_ray_direction);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    discriminant >= 0.0


}