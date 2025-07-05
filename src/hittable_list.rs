use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use std::rc::Rc;

// the hittable list struct
pub struct HittableList{
    objects: Vec<Rc<dyn Hittable>>, // dereference the Rc to get the underlying type
}

impl HittableList
{
    pub fn new() -> Self
    {
        Self{objects: Vec::new()}
    }

    pub fn from(object: Rc<dyn Hittable>) -> Self{
        let mut hittable_list = Self::new();
        hittable_list.add(object);
        hittable_list
    }

    pub fn clear(&mut self)
    {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>)
    {
        self.objects.push(object);
    }

}

impl Hittable for HittableList
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        
        // TODO: corss check this default function
        let mut temp_rec = HitRecord::default();


        // iterate over objects
        // if object is hit set hit_anything to true
        // set closest_so_far to 

        for object in self.objects.iter()
        {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec)
            {
                hit_anything = true;
                closest_so_far = temp_rec.t;

                // this is to prevent dangling references
                *hit_record = temp_rec.clone();
            }

            
        }


        hit_anything
    }
}