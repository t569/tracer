use tracer::{Camera, Sphere};
use tracer::core::Vec3;
use tracer::hittable_list::HittableList;
use std::rc::Rc;
fn main() {
    
    let mut world = HittableList::new();
   
    world.add(Rc::new(
        Sphere::new(Vec3::new(0.0,0.0,-1.0), 0.5)
    ));
     world.add(Rc::new(
        Sphere::new(Vec3::new(0.0,-100.5,-1.0), 100.0)
    ));

    // initialise the camera
    let mut camera = Camera::new(); // all values set to default

    camera.aspect_ratio = 16.0/9.0;
    camera.image_width = 400;

    camera.render(&world);




}

