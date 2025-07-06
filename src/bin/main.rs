use tracer::material::{Lambertian, Metal};
use tracer::{Camera, Sphere};
use tracer::core::Vec3;
use tracer::hittable_list::HittableList;
use std::rc::Rc;
fn main() {
    
    let mut world = HittableList::new();


    // various materials based on the Material trait
    let material_ground = Rc::new(
        Lambertian::new(Vec3::new(0.8, 0.8,0.0)
    ));
    
    let material_center = Rc::new(
        Lambertian::new(Vec3::new(0.1, 0.2,0.5)
    ));
    
    let material_left = Rc::new(
        Metal::new(Vec3::new(0.8, 0.8,0.8)
    ));

    let material_right = Rc::new(
        Metal::new(Vec3::new(0.8, 0.6,0.2)
    ));



   
    world.add(Rc::new(
        Sphere::new(Vec3::new(0.0,-100.5,-1.0), 100.0, material_ground)
    ));
    world.add(Rc::new(
        Sphere::new(Vec3::new(0.0,0.0,-1.2), 0.5, material_center)
    ));
    world.add(Rc::new(
        Sphere::new(Vec3::new(-1.0,0.0,-1.0), 0.5, material_left)
    ));
    world.add(Rc::new(
        Sphere::new(Vec3::new(1.0,0.0,-1.0), 0.5, material_right)
    ));

    // initialise the camera
    let mut camera = Camera::new(); // all values set to default

    camera.aspect_ratio = 16.0/9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.render(&world);




}

