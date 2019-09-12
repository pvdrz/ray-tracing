use crate::bounding_box::BoundingBox;
use crate::hitable::{HitRecord, Hitable};
use crate::material::Material;
use crate::num::*;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere<T: Material> {
    center: Vec3,
    radius: Num,
    material: T,
}

unsafe impl<T: Material> Sync for Sphere<T> {}

impl<T: Material> Sphere<T> {
    pub fn new(center: Vec3, radius: Num, material: T) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl<T: Material> Hitable for Sphere<T> {
    fn hit<'a>(&'a self, r: &Ray, t_min: Num, t_max: Num, rec: &mut HitRecord<'a>) -> bool {
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
                rec.material = &self.material;
                return true;
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at(temp);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.material = &self.material;
                return true;
            }
        }
        false
    }

    fn bounding_box(&self, _: Num, _: Num) -> Option<BoundingBox> {
        Some(BoundingBox::new(
            self.center - self.radius,
            self.center + self.radius,
        ))
    }
}
