use crate::{Ray, HitRecord, core::Vec3};


// this defines an abstract class for objects behaviours with light
pub trait Material 
{
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &Vec3<f64>, scattered: &Ray) -> bool
    {
        false
    }

}

// this is for default value impl, im only using this to compile
pub struct DefaultMaterial;

impl Material for DefaultMaterial{
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &Vec3<f64>, scattered: &Ray) -> bool {
        false
    }
}


