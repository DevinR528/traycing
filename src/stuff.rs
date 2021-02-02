use crate::{Ray, Vec3};

pub trait Hitable {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32, rec: &mut HitRecord) -> bool;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct HitRecord {
    t: f32,
    p: Vec3,
    normal: Vec3,
}

pub struct Objects<H: Hitable>(Vec<H>);

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

pub fn color<H: Hitable>(ray: &Ray, world: H) -> Vec3 {
    let mut rec = HitRecord::default();

    if world.hit(ray, 0.0, f32::MAX, &mut rec) {
        0.5 * Vec3::new(
            rec.normal.x() + 1.0,
            rec.normal.y() + 1.0,
            rec.normal.z() + 1.0,
        )
    } else {
        let unit_dir = ray.direction().normalize();
        let t = 0.5 * (unit_dir.y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hitable for Sphere {
    #[allow(clippy::suspicious_operation_groupings)]
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32, rec: &mut HitRecord) -> bool {
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
                return true;
            }
            let tmp = (-b + (b * b - a * c).sqrt()) / a;
            if tmp < tmax && tmp > tmin {
                rec.t = tmp;
                rec.p = ray.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
        }
        false
    }
}

impl<H: Hitable> Objects<H> {
    pub fn new(items: Vec<H>) -> Self {
        Self(items)
    }
}

impl<H: Hitable> Hitable for &Objects<H> {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32, rec: &mut HitRecord) -> bool {
        let mut tmp_rec = HitRecord::default();
        let mut did_hit = false;
        let mut closest = tmax;

        for obj in &self.0 {
            if obj.hit(ray, tmin, closest, &mut tmp_rec) {
                did_hit = true;
                closest = tmp_rec.t;
                *rec = tmp_rec;
            }
        }
        did_hit
    }
}
