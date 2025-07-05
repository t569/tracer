use std::fs::File;
use std::io::Write;

pub mod core;
pub mod color;
pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod hittable_list;
pub mod interval;
pub mod camera;

pub use core::{Vec3, INFINITY, PI};
pub use color::write_color;
pub use ray::Ray;
pub use hittable::Hittable;
pub use hittable::HitRecord;
pub use sphere::Sphere;
pub use hittable_list::HittableList;
pub use interval::Interval;
pub use camera::Camera;


// util functions
pub fn degrees_to_radians(degrees: f64) -> f64{
    degrees * PI/180.0
}

pub fn write_to_file(filename: &str, data: &str) {
    let mut file = File::create(filename).expect("Unable to create file");
    file.write_all(data.as_bytes()).expect("Unable to write data");
}

pub fn ray_color(ray: &Ray, world: &impl Hittable) -> Vec3<f64> {
    // // Simple ray color function that returns a gradient based on the ray's direction
    // let centre = Vec3::new(0.0, 0.0, -1.0);

    // let t = hit_sphere(&centre, 0.5, ray);

    // if t > 0.0{
    //     let normal = (ray.at(t) - centre).normalize();
    //     return 0.5 * Vec3::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0);
    // }

    // let unit_direction = ray.direction().normalize();

    // let t = 0.5 * (unit_direction.y() + 1.0);
    // Vec3::new(
    //     (1.0 - t) * 1.0 + t * 0.5, // Interpolating between white and light blue
    //     (1.0 - t) * 1.0 + t * 0.7, // Interpolating between white and light blue
    //     (1.0 - t) * 1.0 + t * 1.0, // Interpolating between white and light blue
    // )

    let mut hit_record = HitRecord::default();

    if world.hit(ray, &Interval::new(0.0, INFINITY), &mut hit_record)
    {
        return 0.5 * (hit_record.normal + Vec3::new(1.0, 1.0, 1.0));
    }
    let unit_direction = ray.direction().normalize();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3::new(
        (1.0 - t) * 1.0 + t * 0.5, // Interpolating between white and light blue
        (1.0 - t) * 1.0 + t * 0.7, // Interpolating between white and light blue
        (1.0 - t) * 1.0 + t * 1.0, // Interpolating between white and light blue
    )

}


pub fn hit_sphere(center: &Vec3<f64>, radius: f64, ray: &Ray) -> f64{
    // nahhhh, imma do it my own way

    let oc = center - ray.origin();
    let unit_ray_direction = ray.direction().normalize();

    let a = unit_ray_direction.dot(&unit_ray_direction);
    // let b = -2.0 * oc.dot(&unit_ray_direction);
    let h = oc.dot(&unit_ray_direction);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = h * h -  a * c;

    if discriminant < 0.0 {
        -1.0 // No intersection
    }else{
        h - discriminant.sqrt() / a // Return the distance to the intersection point 
    }


}