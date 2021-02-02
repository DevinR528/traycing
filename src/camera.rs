use rand::random;

use crate::{Ray, Vec3};

pub struct Camera {
    lower_left: Vec3,
    horizon: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            lower_left: Vec3::new(-2.0, -1.0, -1.0),
            horizon: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
            origin: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left + (u * self.horizon) + (v * self.vertical),
        )
    }
}
