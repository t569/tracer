use crate::ray::Ray;
use crate::core::Vec3;




pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}

pub struct HitRecord {
    pub t: f64,
    pub point: Vec3<f64>,
    pub normal: Vec3<f64>,
}


