use crate::hitable::{HitRecord, Hitable};
use crate::num::*;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Default)]
pub struct BoundingBox {
    a: Vec3,
    b: Vec3,
}

impl BoundingBox {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        BoundingBox { a, b }
    }

    pub fn max(&self) -> Vec3 {
        self.a
    }

    pub fn min(&self) -> Vec3 {
        self.b
    }

    pub fn surrounding_box(&self, other: &Self) -> Self {
        let small = self.a.min(&other.a);
        let big = self.b.max(&other.b);

        Self::new(small, big)
    }
}

impl Hitable for BoundingBox {
    fn hit<'a>(&'a self, r: &Ray, t_min: Num, t_max: Num, _: &mut HitRecord<'a>) -> bool {
        let ori = r.origin();
        let dir = r.direction();

        let p = (self.a - ori) / dir;
        let q = (self.b - ori) / dir;

        let (v0, v1) = p.min_max(&q);

        min(v1.x(), t_max) > max(v0.x(), t_min)
            || min(v1.y(), t_max) > max(v0.y(), t_min)
            || min(v1.z(), t_max) > max(v0.z(), t_min)
    }

    fn bounding_box(&self, _: Num, _: Num) -> Option<BoundingBox> {
        Some(self.clone())
    }
}
