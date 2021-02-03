use crate::{
    stuff::{rand_in_unit_sphere, HitRecord},
    Ray, Vec3,
};

pub trait Material {
    fn scatter(
        &self,
        ray: Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}
impl Material for Lambertian {
    fn scatter(
        &self,
        ray: Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let target = rec.p() + rec.normal() + rand_in_unit_sphere();
        *scattered = Ray::new(rec.p(), target - rec.p());
        *attenuation = self.albedo;
        true
    }
}

pub struct Reflective {
    albedo: Vec3,
}

impl Reflective {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
    pub fn reflect(&self, v: Vec3, n: Vec3) -> Vec3 {
        v - (2.0 * v.dot(n) * n)
    }
}
impl Material for Reflective {
    fn scatter(
        &self,
        ray: Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = self.reflect(ray.direction().normalize(), rec.normal());
        *scattered = Ray::new(rec.p(), reflected);
        *attenuation = self.albedo;
        scattered.direction().dot(rec.normal()) > 0.0
    }
}
