use crate::material::{Dummy, Material};
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::num::*;
use crate::bounding_box::BoundingBox;

pub struct HitRecord<'a> {
    pub t: Num,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: Num, p: Vec3, normal: Vec3, material: &'a Material) -> Self {
        HitRecord {
            t,
            p,
            normal,
            material,
        }
    }

    pub fn zero() -> Self {
        Self::new(0.0, Vec3::zero(), Vec3::zero(), &Dummy)
    }
}

pub trait Hitable: Sync {
    fn hit<'a>(&'a self, _: &Ray, _: Num, _: Num, _: &mut HitRecord<'a>) -> bool {
        false
    }

    fn bounding_box(&self, _: Num, _:Num, _: &mut BoundingBox) -> bool {
        false
    }
}
