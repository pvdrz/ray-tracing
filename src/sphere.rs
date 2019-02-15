use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::Num;

pub struct Sphere {
    center: Vec3,
    radius: Num,
}

unsafe impl Sync for Sphere {}

impl Sphere {
    pub fn new(center: Vec3, radius: Num) -> Self {
        Sphere { center, radius }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: Num, t_max: Num, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;

        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at(temp);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at(temp);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
        }
        false
    }
}
