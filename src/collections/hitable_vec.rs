use crate::hitable::*;
use crate::ray::Ray;
use crate::num::Num;
use crate::bounding_box::BoundingBox;

pub struct HitableVec {
    pub inner: Vec<Box<Hitable>>,
}

impl HitableVec {
    pub fn new() -> Self {
        HitableVec { inner: Vec::new() }
    }

    pub fn add<T: 'static + Hitable + Sync>(&mut self, element: T) {
        self.inner.push(Box::new(element))
    }

    pub fn to_vec(self) -> Vec<Box<Hitable>> {
        self.inner
    }
}

impl Hitable for HitableVec {
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

    fn bounding_box(&self, t0: Num, t1:Num, bounding_box: &mut BoundingBox) -> bool {
        if self.inner.len() < 1 {
            return false;
        }
        let mut hitables = self.inner.iter();
        let mut temp_box = BoundingBox::zero();
        if !hitables.next().unwrap().bounding_box(t0, t1, &mut temp_box) {
            return false;
        } else {
            *bounding_box = temp_box.clone();
        }
            for hitable in hitables {
                if hitable.bounding_box(t0, t1, &mut temp_box) {
                    *bounding_box = bounding_box.surrounding_box(&temp_box);
                } else {
                    return false;
                }
            }
            true
    }
}
