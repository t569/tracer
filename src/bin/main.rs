use tracer::material::{Dielectric, Lambertian, Metal};
use tracer::{Camera, Sphere, PI};
use tracer::core::Vec3;
use tracer::hittable_list::HittableList;
use std::rc::Rc;
fn main() {
    
    let mut world = HittableList::new();


    let R = (PI/4 as f64).cos();
    // various materials based on the Material trait
    let material_ground = Rc::new(
        Lambertian::new(Vec3::new(0.8, 0.8,0.0)
    ));
    
    let material_center = Rc::new(
        Lambertian::new(Vec3::new(0.1, 0.2,0.5)
    ));
    
    let material_left = Rc::new(
        Dielectric::new(1.50)   // value of glass refraction index
    );

    let material_inner_bubble =  Rc::new(
        Dielectric::new(1.00/1.50)   // value of glass refraction index of inner glass enclosing air
    );


    // now this is a world filled with water containing an air bubble
    //  let material_left = Rc::new(
    //     Dielectric::new(1.00/1.33)   
    // );

    let material_right = Rc::new(
        Metal::new(Vec3::new(0.8, 0.6,0.2), 1.0
    ));


    // TODO: add a gamma correction toggle
   
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
        Sphere::new(Vec3::new(-1.0,0.0,-1.0), 0.4, material_inner_bubble)
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


    camera.vfov = 120 as f64;
    camera.lookfrom = Vec3::new(-2.0, 2.0, 1.0);
    camera.lookat = Vec3::new(0.0, 0.0, -1.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 10.0;
    camera.focus_dist = 3.4;

    camera.render(&world);




}

