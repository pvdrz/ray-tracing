use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::Num;

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub t: Num,
    pub p: Vec3,
    pub normal: Vec3,
}

impl HitRecord {
    pub fn new(t: Num, p: Vec3, normal: Vec3) -> Self {
        HitRecord { t, p, normal }
    }

    pub fn zero() -> Self {
        Self::new(0.0, Vec3::zero(), Vec3::zero())
    }
}

pub trait Hitable {
    fn hit(&self, _: &Ray, _: Num, _: Num, _: &mut HitRecord) -> bool {
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
    fn hit(&self, r: &Ray, t_min: Num, t_max: Num, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::zero();
        let mut hit_anything = false;
        let mut closest = t_max;
        for hitable in &self.inner {
            if hitable.hit(r, t_min, closest, &mut temp_rec) {
                hit_anything = true;
                closest = temp_rec.t;
                *rec = temp_rec;
            }
        }
        hit_anything
    }
}
