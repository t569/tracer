use std::rc::Rc;

use crate::material::DefaultMaterial;
use crate::ray::Ray;
use crate::core::Vec3;
use crate::interval::Interval;
use crate::Material;


pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: &Interval, hit_record: &mut HitRecord) -> bool;
}

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub point: Vec3<f64>,
    pub mat: Rc<dyn Material>,
    pub normal: Vec3<f64>,
    pub front_face: bool,   // specify if the normal is in the direction of the ray
}

impl HitRecord{
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3<f64>)
    {   
        // front face is opposite ray by default
        self.front_face = ((ray.direction()).dot(outward_normal)) < 0.0;
        
        // set the normal vector direction based on the outward normal passed
        // TODO: check if clone is suitable here
        self.normal = if self.front_face{
            *outward_normal
        }else{
            -*outward_normal
        }

    }

    pub fn default() -> Self
    {
        let def_vec = Vec3::new(0.0, 0.0, 0.0);
        Self { t: 0.0,
            point: def_vec,
            mat: Rc::new(DefaultMaterial{}),
            normal:def_vec,
            front_face: true // by default point against the ray?
        }
    }
}

