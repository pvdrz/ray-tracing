use crate::material::{Dummy, Material};
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::Num;

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

pub trait Hitable {
    fn hit<'a>(&'a self, _: &Ray, _: Num, _: Num, _: &mut HitRecord<'a>) -> bool {
        false
    }
}

pub struct HitableList {
    inner: Vec<Box<dyn Hitable + Sync>>,
}

impl HitableList {
    pub fn new() -> Self {
        HitableList { inner: Vec::new() }
    }

    pub fn add<T: 'static + Hitable + Sync>(&mut self, element: T) {
        self.inner.push(Box::new(element))
    }
}

impl Hitable for HitableList {
    fn hit<'a>(&'a self, r: &Ray, t_min: Num, t_max: Num, rec: &mut HitRecord<'a>) -> bool {
        let mut temp_rec = HitRecord::zero();
        let mut hit_anything = false;
        let mut closest = t_max;
        for hitable in &self.inner {
            if hitable.hit(r, t_min, closest, &mut temp_rec) {
                hit_anything = true;
                closest = temp_rec.t;
            }
        }
        if hit_anything {
            *rec = temp_rec;
        }
        hit_anything
    }
}
