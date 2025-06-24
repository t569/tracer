use crate::core::Vec3;

pub struct Ray {
    origin: Vec3<f64>,
    direction: Vec3<f64>,
}

impl Ray {
    pub fn new(origin: Vec3<f64>, direction: Vec3<f64>) -> Self {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> &Vec3<f64> {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3<f64> {
        &self.direction
    }

    pub fn at(&self, t: f64) -> Vec3<f64> {
        self.origin + self.direction * t
    }
}