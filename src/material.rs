use crate::{core::Vec3, random_float, HitRecord, Ray};

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
            true => fuzz,
            false => 1.0
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

pub struct Dielectric
{
    refraction_index: f64,
}

impl Dielectric
{
    pub fn new(refract_index: f64) -> Self
    {
        Self{refraction_index: refract_index}
    }

    // schlick approximation: glass and other dielectrics behave like mirrors at steep angles
    fn reflectance(cosine: f64, refraction_index: f64) -> f64
    {
        let mut r0 = (1 as f64 - refraction_index)/ (1 as f64 + refraction_index);
        r0 = r0 * r0;

        r0 + (1 as f64 - r0) * (1 as f64 - cosine).powf(5 as f64)
    }
}

impl Material for Dielectric
{
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3<f64>, scattered: &mut Ray) -> bool {
        // this is always 1, the surface absorbs nothing
        *attenuation = Vec3::new(1.0, 1.0, 1.0);

        // this is based on the change of perspective based on what medium we are in
        let ref_index = match hit_record.front_face {
            true => 1.0/self.refraction_index,
            false => self.refraction_index,
        };

        let unit_direction = ray_in.direction().normalize();

        // the angles being computed
        let cos_theta = Vec3::dot_explicit(&-unit_direction, &hit_record.normal);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        // this is for total internal reflection
        let cannot_refract = ref_index * sin_theta > 1.0;
        let mut direction = Vec3::origin();

        // the reflectance code checks if we're in the range of numbers where reflectance can take place
        if cannot_refract || Self::reflectance(cos_theta, ref_index) > random_float()
        {
            direction = Vec3::reflect(&unit_direction, &hit_record.normal);
        }
        else {
            direction = Vec3::refract(&unit_direction, &hit_record.normal, ref_index);
        }

        *scattered = Ray::new(hit_record.point, direction);
        true
    }
}






