use crate::hitable::{HitRecord, Hitable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::num::*;
use crate::bounding_box::BoundingBox;

pub struct Triangle<T: Material> {
    normal: Vec3,
    p1: Vec3,
    p2: Vec3,
    p3: Vec3,
    material: T,
}

unsafe impl<T: Material> Sync for Triangle<T> {}

impl<T: Material> Triangle<T> {
    pub fn new(normal: Vec3, p1: Vec3, p2: Vec3, p3: Vec3, material: T) -> Self {
        Triangle {
            normal,
            p1,
            p2,
            p3,
            material,
        }
    }
    pub fn from_points(p1: Vec3, p2: Vec3, p3: Vec3, material: T) -> Self {
        let normal = (p2 - p1).cross(p3 - p1).unit();
        Triangle {
            normal,
            p1,
            p2,
            p3,
            material,
        }
    }
}

impl<T: Material> Hitable for Triangle<T> {
    fn hit<'a>(&'a self, r: &Ray, t_min: Num, t_max: Num, rec: &mut HitRecord<'a>) -> bool {
        let a = self.normal.dot(r.direction());

        if a != 0.0 {
            let b = self.normal.dot(r.origin());
            let c = self.normal.dot(self.p1);

            let temp = (c - b) / a;
            let p = r.point_at(temp);

            if temp < t_max && temp > t_min {
                let u = self.p2 - self.p1;
                let v = self.p3 - self.p1;
                let w = p - self.p1;
                let n = u.cross(v);
                let norm = n.dot(n);

                let gamma = u.cross(w).dot(n) / norm;
                if gamma < 0.0 || gamma > 1.0 {
                    return false;
                }

                let beta = w.cross(v).dot(n) / norm;
                if beta < 0.0 || beta > 1.0 {
                    return false;
                }

                let alpha = 1.0 - gamma - beta;
                if alpha < 0.0 || alpha > 1.0 {
                    return false;
                }

                rec.t = temp;
                rec.p = p;
                rec.normal = self.normal;
                rec.material = &self.material;

                return true;
            }
        }
        false
    }

    fn bounding_box(&self, _: Num, _:Num, bounding_box: &mut BoundingBox) -> bool {
        let x_max = self.p1.x().max(self.p2.x()).max(self.p3.x());
        let x_min = self.p1.x().min(self.p2.x()).min(self.p3.x());
        let y_max = self.p1.y().max(self.p2.y()).max(self.p3.y());
        let y_min = self.p1.y().min(self.p2.y()).min(self.p3.y());
        let z_max = self.p1.z().max(self.p2.z()).max(self.p3.z());
        let z_min = self.p1.z().min(self.p2.z()).min(self.p3.z());
        *bounding_box = BoundingBox::new(
            Vec3::new(x_min, y_min, z_min),
            Vec3::new(x_max, y_max, z_max),
                                        );
        true
    }
}
