use tracer::material::{Dielectric, Lambertian, Metal};
use tracer::{random_float, random_float_interval, Camera, Sphere, PI};
use tracer::core::Vec3;
use tracer::hittable_list::HittableList;
use tracer::material::Material;
use std::rc::Rc;
fn main() {
    
    let mut world = HittableList::new();

    // // various materials based on the Material trait
    // let material_ground = Rc::new(
    //     Lambertian::new(Vec3::new(0.8, 0.8,0.0)
    // ));
    
    // let material_center = Rc::new(
    //     Lambertian::new(Vec3::new(0.1, 0.2,0.5)
    // ));
    
    // let material_left = Rc::new(
    //     Dielectric::new(1.50)   // value of glass refraction index
    // );

    // let material_inner_bubble =  Rc::new(
    //     Dielectric::new(1.00/1.50)   // value of glass refraction index of inner glass enclosing air
    // );


    // // now this is a world filled with water containing an air bubble
    // //  let material_left = Rc::new(
    // //     Dielectric::new(1.00/1.33)   
    // // );

    // let material_right = Rc::new(
    //     Metal::new(Vec3::new(0.8, 0.6,0.2), 1.0
    // ));


    // // TODO: add a gamma correction toggle
   
    // world.add(Rc::new(
    //     Sphere::new(Vec3::new(0.0,-100.5,-1.0), 100.0, material_ground)
    // ));
    // world.add(Rc::new(
    //     Sphere::new(Vec3::new(0.0,0.0,-1.2), 0.5, material_center)
    // ));
    // world.add(Rc::new(
    //     Sphere::new(Vec3::new(-1.0,0.0,-1.0), 0.5, material_left)
    // ));

    // world.add(Rc::new(
    //     Sphere::new(Vec3::new(-1.0,0.0,-1.0), 0.4, material_inner_bubble)
    // ));
    // world.add(Rc::new(
    //     Sphere::new(Vec3::new(1.0,0.0,-1.0), 0.5, material_right)
    // ));

    // // initialise the camera
    // let mut camera = Camera::new(); // all values set to default

    // camera.aspect_ratio = 16.0/9.0;
    // camera.image_width = 400;
    // camera.samples_per_pixel = 100;
    // camera.max_depth = 50;


    // camera.vfov = 120 as f64;
    // camera.lookfrom = Vec3::new(-2.0, 2.0, 1.0);
    // camera.lookat = Vec3::new(0.0, 0.0, -1.0);
    // camera.vup = Vec3::new(0.0, 1.0, 0.0);

    // camera.defocus_angle = 10.0;
    // camera.focus_dist = 3.4;

    // Render a BUT LOAD of spheres!!

    // Ground material
    let ground_material = Rc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material.clone(),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_float();
            let center = Vec3::new(
                a as f64 + 0.9 * random_float(),
                0.2,
                b as f64 + 0.9 * random_float(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).mag() > 0.9 {
                let sphere_material: Rc<dyn Material>;

                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Vec3::random_vec3() * Vec3::random_vec3();
                    sphere_material = Rc::new(Lambertian::new(albedo));

                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Vec3::random_range_vec3(0.5, 1.0);
                    let fuzz = random_float_interval(0.0, 0.5);
                    sphere_material = Rc::new(Metal::new(albedo, fuzz));
                } else {
                    // Glass
                    sphere_material = Rc::new(Dielectric::new(1.5));
                }

                world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    // 3 big spheres
    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Rc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3)));

    // Camera setup
    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Vec3::new(13.0, 2.0, 3.0);
    cam.lookat = Vec3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;


    cam.render(&world);

}

