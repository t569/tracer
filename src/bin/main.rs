use tracer::{write_to_file, Sphere};
use tracer::core::Vec3;
use tracer::color::write_color;
use tracer::hittable_list::HittableList;
use std::cmp::Ordering;
use std::rc::Rc;
fn main() {
    
    // IMAGE



    // lets define a simple camera

    // aspect ratio
    let aspect_ratio: f64 = 16.0/9.0;      // most common aspect ratio

    // image dimensions
    let image_width = 400;
    let mut image_height = (image_width as f64 / aspect_ratio) as u32;


    // wtf, borrowing 1???
    image_height = match image_height.cmp(&1){
        Ordering::Less => 1, // Ensure height is at least 1
        _ => image_height,   // Keep the original height if it's greater than or equal to 1
    };

    // WORLD/SCENE
    let mut world = HittableList::new();
    world.add(Rc::new(
        Sphere::new(Vec3::new(0.0,0.0,-1.0), 0.5)
    ));
     world.add(Rc::new(
        Sphere::new(Vec3::new(0.0,-100.5,-1.0), 100.0)
    ));




    // CAMERA
    // finally our camera


    // we use the idea of a pinhole camera model
    // so basically, the viewport a plane of points through which rays pass through
    // then the rays are cast from the camera through the viewport to the scene
    // and that scene is the image where pixels are rendered

    // camera parameters

    let focal_length = 1.0; // distance from the camera to the image plane
    let viewport_height: f64 = 2.0; // height of the viewport
    let viewport_width = aspect_ratio * viewport_height; // width of the viewport
    let camera_centre = Vec3::origin(); // camera is at the origin

    // the vectors across the viewport
    let viewport_horizontal_u = Vec3::new(viewport_width, 0.0, 0.0); // horizontal vector across the viewport
    let viewport_vertical_v = Vec3::new(0.0,-viewport_height, 0.0); // vertical vector across the viewport

    // the delta vectors of the viewport
    // these are the vectors that define the viewport in 3D space

    let viewport_delta_u = viewport_horizontal_u / (image_width as f64); // delta u vector
    let viewport_delta_v = viewport_vertical_v / (image_height as f64); // delta v vector


    let viewport_00_origin = camera_centre - viewport_horizontal_u / 2.0 - viewport_vertical_v / 2.0 - Vec3::new(0.0, 0.0, focal_length);
    

    // now for our image stuff with the camera logic

    // the offset of the first point is half the distance of the delta vectors
    // this is to center the first pixel in the viewport
    // so we can get the pixel at (0, 0) in the viewport
    let pixel_00_origin_loc = viewport_00_origin + viewport_delta_u / 2.0 + viewport_delta_v / 2.0; 




    // RENDERING



    // write this all to a file

    // first put all of this into a string
    let mut image_data = String::new();

    image_data.push_str("P3\n");
    image_data.push_str(&format!("{} {}\n", image_width, image_height));
    image_data.push_str("255\n");

    for i in 0..image_height{

        // show the progress in the grip rendering
        println!("Progress: {}/{}", i, image_height);

        for j in 0..image_width{
            // // Generate a pixel value based on its position
            // let r = ((i as f32 / image_height as f32) * 255.0) as u8;
            // let g = ((j as f32 / image_width as f32) * 255.0) as u8;
            // let b = (0.0 * 255.0) as u8; // No blue component
            // let r = i / (image_width - 1);
            // let g = j / (image_height - 1);
            // let pixel_color = Vec3::new(r as f32, g as f32, 0.0);

            // get the pixel centres

            // lets be sane about the implementation
            let pixel_centre = pixel_00_origin_loc + (viewport_delta_v * i as f64) + (viewport_delta_u * j as f64);


            // create a ray from the camera to the pixel centre
            let ray = tracer::Ray::new(camera_centre, pixel_centre - camera_centre);

            // finally the color
            let pixel_color = tracer::ray_color(&ray, &world);
            image_data.push_str(&write_color(&pixel_color, 1));

        }
    }
    println!("Rendering complete, writing to file...");
    write_to_file("renders/image.ppm", &image_data);

}

