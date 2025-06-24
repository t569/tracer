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
    

    let unit_direction = ray.direction().normalize();

    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3::new(
        (1.0 - t) * 1.0 + t * 0.5, // Interpolating between white and light blue
        (1.0 - t) * 1.0 + t * 0.7, // Interpolating between white and light blue
        (1.0 - t) * 1.0 + t * 1.0, // Interpolating between white and light blue
    )
}