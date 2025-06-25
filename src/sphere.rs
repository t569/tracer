use crate::core::Vec3;
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;

pub struct Sphere {
    center: Vec3<f64>,
    radius: f64,
}

// remember to use Hittable trait in code, we import  it from hittable.rs
impl Hittable for Sphere{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord)-> bool {
        let oc = ray.origin() - &self.center;
        let a = ray.direction().dot(ray.direction());
        let h = oc.dot(ray.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false; // No intersection
        }

        let sqrt_d = discriminant.sqrt();
        let root = (h - sqrt_d) / a;

        if root < t_min || root > t_max {
            return false; // Intersection not in range
        }

        hit_record.t = root;
        hit_record.point = ray.at(root);

        // Calculate the normal at the intersection point
        hit_record.normal = (hit_record.point - self.center).normalize();

        true
    }
}