use crate::vec::Vec3;

pub struct Ray {
    a: Vec3,
    b: Vec3,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self { a, b }
    }

    pub fn origin(&self) -> Vec3 {
        self.a
    }

    pub fn direction(&self) -> Vec3 {
        self.b
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.a + t * self.b
    }
}

fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(ray.direction());
    let b = 2.0 * oc.dot(ray.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

pub fn color(ray: &Ray) -> Vec3 {
    let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, ray);

    if t > 0.0 {
        let n = (ray.point_at_parameter(t) - Vec3::new(0.0, 0.0, -1.0)).normalize();
        return 0.5 * Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    let unit_dir = ray.direction().normalize();
    let t = 0.5 * (unit_dir.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}
