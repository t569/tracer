use crate::{Ray, HitRecord, core::Vec3};

// personally feel there should be a universal materical class fr

// this defines an abstract class for objects behaviours with light
pub trait Material 
{
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3<f64>, scattered: &mut Ray) -> bool
    {
        false
    }
}

// this is for default value impl, im only using this to compile
pub struct DefaultMaterial;

impl Material for DefaultMaterial{
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3<f64>, scattered: &mut Ray) -> bool {
        false
    }
}

// albedo defines a sort of fractional reflectance, the percentage of rays reflected/absorbed off a body
pub struct Lambertian
{
    albedo: Vec3<f64>,
}

impl Lambertian
{
    pub fn new(albedo: Vec3<f64>) -> Self
    {
        Self{
            albedo: albedo
        }
    }
}

impl Material for Lambertian
{
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3<f64>, scattered: &mut Ray) -> bool {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();
        
        // catch degenerate near zero values, when normal and random unit vector are almost in opposite direction
        if scatter_direction.near_zero()
        {
            scatter_direction = hit_record.normal;
        }
        // apparently we're passing these values in to be changed
        *scattered = Ray::new(hit_record.point, scatter_direction);
        *attenuation = self.albedo.clone();
        true
    }
}

pub struct Metal
{
    albedo: Vec3<f64>,
    fuzz: f64,
}

impl Metal
{
    pub fn new(albedo: Vec3<f64>, fuzz: f64) -> Self
    {
        // this implements a sphere which causes displacement of the reflection vectors
        let fuzz = match fuzz < 1.0 {
            true => 1.0,
            false => fuzz
        };
        Self {albedo: albedo, fuzz: fuzz}
    }
}

impl Material for Metal
{
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3<f64>, scattered: &mut Ray) -> bool {
        let mut reflected = Vec3::reflect(ray_in.direction(), &hit_record.normal);


        // implement the fuzzing of the rays.
        // we need to scale (to unit vector) each fuzz sphere to be consistent when compared with the reflection vector
        reflected = reflected.normalize() + (self.fuzz * Vec3::random_unit_vector());

        *scattered = Ray::new(hit_record.point, reflected);
        *attenuation = self.albedo.clone();

        // if we are scatter below thw surface because of a big sphere or a surface ray, simply absorb it
        hit_record.normal.dot(&scattered.direction()) > 0.0
    }
}






