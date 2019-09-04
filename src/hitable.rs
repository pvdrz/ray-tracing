use crate::bounding_box::BoundingBox;
use crate::material::{Dummy, Material};
use crate::num::*;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord<'a> {
    pub t: Num,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: Num, p: Vec3, normal: Vec3, material: &'a dyn Material) -> Self {
        HitRecord {
            t,
            p,
            normal,
            material,
        }
    }
}

impl<'a> Default for HitRecord<'a> {
    fn default() -> Self {
        Self::new(
            Default::default(),
            Default::default(),
            Default::default(),
            &Dummy,
        )
    }
}

pub trait Hitable: Sync {
    fn hit<'a>(&'a self, _: &Ray, _: Num, _: Num, _: &mut HitRecord<'a>) -> bool {
        false
    }

    fn bounding_box(&self, _: Num, _: Num) -> Option<BoundingBox> {
        None
    }
}
