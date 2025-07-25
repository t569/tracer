use crate::{degrees_to_radians, hittable::{self, HitRecord, Hittable}, random_float, Ray, INFINITY};
use crate::core::Vec3;
use crate::interval::Interval;
use crate::color::write_color;
use std::cmp::Ordering;

pub struct Camera{
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,             // max number of ray bounces in ray color function
    pub vfov: f64,                  // vertical field of view for the camera
    pub vup: Vec3<f64>,             // realtive up direction for our camera
    pub lookfrom: Vec3<f64>,        // point the camera is looking from
    pub lookat: Vec3<f64>,          // point the camera is looking at: TODO specify this as a direction: just put lookat at the centre of the screen or sm
    pub defocus_angle: f64,
    pub focus_dist: f64,


    // private fields
    image_height: u32,              // Rendered image height
    pixel_samples_scale: f64,       // Color scale factor for a sum of pixel samples
    centre: Vec3<f64>,              // Camera center
    pixel_00_origin_loc: Vec3<f64>, // Location of pixel 0,0
    pixel_delta_u: Vec3<f64>,       // Offset to pixel to the right
    pixel_delta_v: Vec3<f64>,       // Offset to pixel below
    // camera frame basis vectros
    u: Vec3<f64>,
    v: Vec3<f64>,
    w: Vec3<f64>,
    defocus_disk_u: Vec3<f64>,      // Defocus disk horizontal radius
    defocus_disk_v: Vec3<f64>,      // Defocus disk vertical radius

}


impl Camera{
    pub fn new() -> Self
    {
        Self
        {
            aspect_ratio: 0.0,
            image_width: 0,
            samples_per_pixel: 1,
            max_depth: 10,
            vfov: 90 as f64,
            lookfrom: Vec3::origin(),
            lookat: Vec3::new(0.0, 0.0,-1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0 as f64,
            focus_dist: 10 as f64,
            centre: Vec3::origin(),
            image_height: 0,
            pixel_samples_scale: 0.5,
            pixel_00_origin_loc: Vec3::origin(),
            pixel_delta_u: Vec3::origin(),
            pixel_delta_v: Vec3::origin(),

            // these dont do anything for now
            u: Vec3::origin(),
            v: Vec3::origin(),
            w: Vec3::origin(),
            defocus_disk_u: Vec3::origin(),
            defocus_disk_v: Vec3::origin(),
        }
    }
    pub fn render(&mut self, world: & impl Hittable)
    {
        self.initialize();

         let mut image_data = String::new();

        image_data.push_str("P3\n");
        image_data.push_str(&format!("{} {}\n", self.image_width, self.image_height));
        image_data.push_str("255\n");

        for i in 0..self.image_height{

            // show the progress in the grip rendering
            println!("Progress: {}/{}", i, self.image_height);

            for j in 0..self.image_width{

                let mut pixel_color = Vec3::origin();

                for _k in 0..self.samples_per_pixel
                {
                    let r = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&r,self.max_depth, world);
                }
                image_data.push_str(&write_color(&(pixel_color * self.pixel_samples_scale)));

            }
        }

        println!("Rendering complete, writing to file...");
        crate::write_to_file("renders/image.ppm", &image_data);

    }

    fn ray_color(ray: &Ray, depth: u32, world: &impl Hittable) -> Vec3<f64>
    {
        // if depth is zero, return black to avoid infinite bounces
        if depth <= 0
        {
            return Vec3::origin();
        }
        let mut hit_record = HitRecord::default();


        // if we hit something, generate a new ray in a random direction and trace where it goes and do this recursively

        // it is not set at 0.0 but at 0.0001 to prevent shadow acne (near hits)
        if world.hit(ray, &Interval::new(0.0001, INFINITY), &mut hit_record)
        {
            let mut attenuation = Vec3::origin();
            // ray scattered;
            let mut scattered = Ray::new(Vec3::origin(), Vec3::origin());

            if hit_record.mat.scatter(ray, &hit_record, &mut attenuation, &mut scattered)
            {
                return attenuation * Self::ray_color(&scattered, depth - 1, world);
            }


            // this is synonymous to black
            return Vec3::origin();
        }


        let unit_direction = ray.direction().normalize();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Vec3::new(
            (1.0 - t) * 1.0 + t * 0.5, // Interpolating between white and light blue
            (1.0 - t) * 1.0 + t * 0.7, // Interpolating between white and light blue
            (1.0 - t) * 1.0 + t * 1.0, // Interpolating between white and light blue
        )
    }

    fn initialize(&mut self)
    {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

         // wtf, borrowing 1???
        self.image_height = match self.image_height.cmp(&1){
            Ordering::Less => 1, // Ensure height is at least 1
            _ => self.image_height,   // Keep the original height if it's greater than or equal to 1
        };

        // this is the camera centre
        self.centre = self.lookfrom;

        // viewport dimensions
        // let focal_length = (self.lookfrom - self.lookat).mag();
        let theta = crate::degrees_to_radians(self.vfov);
        let h = (theta/ 2.0).tan();

        // calculate the u, v, w unit basis  vectors for the camera coordinate frame
        self.w = (self.lookfrom - self.lookat).normalize();
        self.u = self.vup.cross(&self.w);
        self.v = self.w.cross(&self.u);


        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64) / self.image_height as f64;
        

        // calculate vectors across the horizontal and down the vertical viewport edges
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;


        // calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v/ self.image_height as f64;

        // calculate the location of the upper left pixel
        let viewport_00_origin = self.centre - (self.w * self.focus_dist)- viewport_u / 2.0 - viewport_v / 2.0;
    
        // the offset of the first point is half the distance of the delta vectors
        // this is to center the first pixel in the viewport
        // so we can get the pixel at (0, 0) in the viewport
        self.pixel_00_origin_loc = viewport_00_origin + self.pixel_delta_u / 2.0 + self.pixel_delta_v / 2.0; 


        // calculate the camera defocus disk basis vectors
        let defocus_radius = self.focus_dist * degrees_to_radians(self.defocus_angle/ 2 as f64);
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;

    }

    fn get_ray(&self, i: u32, j: u32) -> Ray
    {
        // construct a camera ray originating fomr the defocus disk and directed at a randomly sampled point around the pisel location i,j
        let offset = Self::sample_square();

        let pixel_sample = self.pixel_00_origin_loc 
        + ((offset.x() + (i as f64)) * self.pixel_delta_v)
        + ((offset.y() + (j as f64)) * self.pixel_delta_u);

        let ray_origin = match self.defocus_angle <= 0.0{
            true => self.centre,
            false => self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vec3<f64>{
        // return a random point in the [-0.5, -0.5] [0.5, 0.5] square
        Vec3::new(random_float() - 0.5, random_float() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Vec3<f64>
    {
        // return a random point in the camera defocus disk
        let p = Vec3::random_in_unit_disk();
        self.centre + (self.defocus_disk_u * *p.x()) + (self.defocus_disk_v * *p.y()) 
    }
}
