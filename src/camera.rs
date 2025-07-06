use crate::{hittable::{self, HitRecord, Hittable}, random_float, Ray, INFINITY};
use crate::core::Vec3;
use crate::interval::Interval;
use crate::color::write_color;
use std::cmp::Ordering;

pub struct Camera{
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,             // max number of ray bounces in ray color function

    // private fields
    image_height: u32,              // Rendered image height
    pixel_samples_scale: f64,       // Color scale factor for a sum of pixel samples
    centre: Vec3<f64>,              // Camera center
    pixel_00_origin_loc: Vec3<f64>, // Location of pixel 0,0
    pixel_delta_u: Vec3<f64>,       // Offset to pixel to the right
    pixel_delta_v: Vec3<f64>,       // Offset to pixel below
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
            centre: Vec3::origin(),
            image_height: 0,
            pixel_samples_scale: 0.5,
            pixel_00_origin_loc: Vec3::origin(),
            pixel_delta_u: Vec3::origin(),
            pixel_delta_v: Vec3::origin(),
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

        self.centre = Vec3::origin();

        // viewport dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64) / self.image_height as f64;
        

        // calculate vectors across the horizontal and down the vertical viewport edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v/ self.image_height as f64;

        // calculate the location of the upper left pixel
        let viewport_00_origin = self.centre - viewport_u / 2.0 - viewport_v / 2.0 - Vec3::new(0.0, 0.0, focal_length);
    
        // the offset of the first point is half the distance of the delta vectors
        // this is to center the first pixel in the viewport
        // so we can get the pixel at (0, 0) in the viewport
        self.pixel_00_origin_loc = viewport_00_origin + self.pixel_delta_u / 2.0 + self.pixel_delta_v / 2.0; 

    }

    fn get_ray(&self, i: u32, j: u32) -> Ray
    {
        let offset = Self::sample_square();

        let pixel_sample = self.pixel_00_origin_loc 
        + ((offset.x() + (i as f64)) * self.pixel_delta_v)
        + ((offset.y() + (j as f64)) * self.pixel_delta_u);

        let ray_origin = self.centre;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vec3<f64>{
        // return a random point in the [-0.5, -0.5] [0.5, 0.5] square
        Vec3::new(random_float() - 0.5, random_float() - 0.5, 0.0)
    }
}
