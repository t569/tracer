use crate::core::Vec3;

struct Ray {
    origin: Vec3<f32>,
    direction: Vec3<f32>,
}

impl Ray {
    pub fn new(origin: Vec3<f32>, direction: Vec3<f32>) -> Self {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> &Vec3<f32> {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3<f32> {
        &self.direction
    }

    pub fn at(&self, t: f32) -> Vec3<f32> {
        self.origin + self.direction * t
    }
}