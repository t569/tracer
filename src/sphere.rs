use crate::core::Vec3;
use crate::hittable::{Hittable, HitRecord};
use crate::interval::Interval;
use crate::ray::Ray;

pub struct Sphere {
    center: Vec3<f64>,
    radius: f64,
}

impl Sphere{
    pub fn new(center: Vec3<f64>, radius: f64) -> Self{
        Self{center, radius}
    }
}
// remember to use Hittable trait in code, we import  it from hittable.rs
impl Hittable for Sphere{
    fn hit(&self, ray: &Ray,interval: &Interval, hit_record: &mut HitRecord)-> bool {
       
        let oc = &self.center - ray.origin();


        let a = ray.direction().dot(ray.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
        let h = ray.direction().dot(&oc);
        let h_discriminant = h * h - a * c;

        if h_discriminant < 0.0 {
            return false; // No intersection
        }

        let sqrt_d = h_discriminant.sqrt();
        let mut root = (h - sqrt_d) / a;

        if !interval.surrounds(root)
        {
            root = (h + sqrt_d)/a;

            if !interval.surrounds(root)
            {
                return false;
            }
        }
        
        hit_record.t = root;
        hit_record.point = ray.at(root);

        // Calculate the normal at the intersection point
        let  outward_normal = (hit_record.point - self.center)/self.radius ;
        hit_record.set_face_normal(&ray, &outward_normal);
        true
    }
}