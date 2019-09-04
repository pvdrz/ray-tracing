use crate::bounding_box::BoundingBox;
use crate::hitable::*;
use crate::num::Num;
use crate::ray::Ray;

#[derive(Default)]
pub struct HitableVec {
    pub inner: Vec<Box<dyn Hitable>>,
}

impl HitableVec {
    pub fn add<T: 'static + Hitable + Sync>(&mut self, element: T) {
        self.inner.push(Box::new(element))
    }

    pub fn into_vec(self) -> Vec<Box<dyn Hitable>> {
        self.inner
    }
}

impl Hitable for HitableVec {
    fn hit<'a>(&'a self, r: &Ray, t_min: Num, t_max: Num, rec: &mut HitRecord<'a>) -> bool {
        let mut temp_rec = HitRecord::default();
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

    fn bounding_box(&self, t0: Num, t1: Num) -> Option<BoundingBox> {
        let mut hitables = self.inner.iter();
        let mut temp_box = hitables.next()?.bounding_box(t0, t1)?;
        let mut bounding_box = temp_box.clone();
        for hitable in hitables {
            temp_box = hitable.bounding_box(t0, t1)?;
            bounding_box = bounding_box.surrounding_box(&temp_box);
        }
        Some(bounding_box)
    }
}
