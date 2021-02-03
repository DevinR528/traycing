use rand::random;

use crate::{material::Material, Ray, Vec3};

pub trait Hitable {
    fn hit(&self, ray: Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
}

pub struct HitRecord<'a> {
    t: f32,
    p: Vec3,
    normal: Vec3,
    material: &'a dyn Material,
}

#[derive(Default)]
pub struct Objects<H: Hitable>(Vec<H>);
unsafe impl<H: Hitable> Sync for Objects<H> {}

pub struct Sphere<'a> {
    center: Vec3,
    radius: f32,
    material: &'a dyn Material,
}

pub fn color<H: Hitable>(ray: Ray, world: H, depth: u8) -> Vec3 {
    // `world.hit()` fills in our `rec` HitRecord struct for us
    match world.hit(ray, 0.001, f32::MAX) {
        Some(rec) => {
            let mut scattered = Ray::default();
            let mut attenuation = Vec3::default();
            if depth < 50
                && rec
                    .material
                    .scatter(ray, &rec, &mut attenuation, &mut scattered)
            {
                attenuation * color(scattered, world, depth + 1)
            } else {
                Vec3::default()
            }
        }
        None => {
            let unit_dir = ray.direction().normalize();
            let t = 0.5 * (unit_dir.y() + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

impl<'a> HitRecord<'a> {
    pub fn new(material: &'a dyn Material) -> Self {
        Self {
            material,
            p: Vec3::default(),
            t: Default::default(),
            normal: Vec3::default(),
        }
    }
    pub fn p(&self) -> Vec3 {
        self.p
    }
    pub fn normal(&self) -> Vec3 {
        self.normal
    }
}
impl<'a> Sphere<'a> {
    pub fn new(center: Vec3, radius: f32, material: &'a dyn Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl<'a> Hitable for Sphere<'a> {
    #[allow(clippy::suspicious_operation_groupings)]
    fn hit(&self, ray: Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        let mut rec = HitRecord::new(self.material);
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(ray.direction());
        let b = oc.dot(ray.direction());
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let tmp = (-b - (b * b - a * c).sqrt()) / a;

            if tmp < tmax && tmp > tmin {
                rec.t = tmp;
                rec.p = ray.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return Some(rec);
            }
            let tmp = (-b + (b * b - a * c).sqrt()) / a;
            if tmp < tmax && tmp > tmin {
                rec.t = tmp;
                rec.p = ray.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return Some(rec);
            }
        }
        None
    }
}

impl<H: Hitable> Objects<H> {
    pub fn new(items: Vec<H>) -> Self {
        Self(items)
    }
}

impl<H: Hitable> Hitable for &Objects<H> {
    fn hit(&self, ray: Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        let mut tmp_rec = None;
        let mut closest = tmax;

        for obj in &self.0 {
            if let Some(rec) = obj.hit(ray, tmin, closest) {
                closest = rec.t;
                tmp_rec = Some(rec);
            }
        }
        tmp_rec
    }
}

pub fn rand_in_unit_sphere() -> Vec3 {
    let mut p = Vec3::default();
    loop {
        p = 2.0 * Vec3::new(rand::random(), rand::random(), random()) - Vec3::new(1.0, 1.0, 1.0);
        if p.len_sqrd() >= 1.0 {
            break;
        }
    }
    p
}
