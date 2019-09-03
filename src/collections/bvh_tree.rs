use crate::bounding_box::BoundingBox;
use crate::hitable::*;
use crate::num::{Int, Num};
use crate::ray::Ray;

use rand::prelude::*;

use std::cmp::Ordering;

pub struct BVHTree {
    left: Option<Box<dyn Hitable>>,
    right: Option<Box<dyn Hitable>>,
    bounding_box: BoundingBox,
}

unsafe impl Sync for BVHTree {}

impl BVHTree {
    pub fn new(mut l: Vec<Box<dyn Hitable>>, t0: Num, t1: Num, rng: &mut ThreadRng) -> Self {
        let n = l.len();

        match (3.0 * rng.gen::<Num>()) as Int {
            0 => l.sort_by(cmp_x),
            1 => l.sort_by(cmp_y),
            _ => l.sort_by(cmp_z),
        };

        match n {
            0 => unreachable!(),
            1 => {
                let left = l.pop().unwrap();
                let bounding_box = left.bounding_box(t0, t1).unwrap_or_default();
                BVHTree {
                    left: Some(left),
                    right: None,
                    bounding_box,
                }
            }
            2 => {
                let left = l.pop().unwrap();
                let right = l.pop().unwrap();
                let box_left = left.bounding_box(t0, t1).unwrap_or_default();
                let box_right = right.bounding_box(t0, t1).unwrap_or_default();
                let bounding_box = box_left.surrounding_box(&box_right);
                BVHTree {
                    left: Some(left),
                    right: Some(right),
                    bounding_box,
                }
            }
            _ => {
                let l_right = l.split_off(n / 2);
                let left = Self::new(l, t0, t1, rng);
                let right = Self::new(l_right, t0, t1, rng);
                let box_left = left.bounding_box(t0, t1).unwrap_or_default();
                let box_right = right.bounding_box(t0, t1).unwrap_or_default();
                let bounding_box = box_left.surrounding_box(&box_right);
                BVHTree {
                    left: Some(Box::new(left)),
                    right: Some(Box::new(right)),
                    bounding_box,
                }
            }
        }
    }
}

impl Hitable for BVHTree {
    fn hit<'a>(&'a self, r: &Ray, t_min: Num, t_max: Num, rec: &mut HitRecord<'a>) -> bool {
        let mut left_rec = HitRecord::zero();
        let mut right_rec = HitRecord::zero();

        let hit_left = match &self.left {
            Some(left) => left.hit(r, t_min, t_max, &mut left_rec),
            None => false,
        };
        let hit_right = match &self.right {
            Some(right) => right.hit(r, t_min, t_max, &mut right_rec),
            None => false,
        };

        if hit_left && hit_right {
            if left_rec.t < right_rec.t {
                *rec = left_rec;
            } else {
                *rec = right_rec;
            }
            true
        } else if hit_left {
            *rec = left_rec;
            true
        } else if hit_right {
            *rec = right_rec;
            true
        } else {
            false
        }
    }

    fn bounding_box(&self, _: Num, _: Num) -> Option<BoundingBox> {
        Some(self.bounding_box.clone())
    }
}

fn cmp_x(a: &Box<dyn Hitable>, b: &Box<dyn Hitable>) -> Ordering {
    let box_left = a.bounding_box(0.0, 0.0).unwrap_or_default();
    let box_right = b.bounding_box(0.0, 0.0).unwrap_or_default();
    box_left
        .min()
        .x()
        .partial_cmp(&box_right.min().x())
        .unwrap_or(Ordering::Equal)
}

fn cmp_y(a: &Box<dyn Hitable>, b: &Box<dyn Hitable>) -> Ordering {
    let box_left = a.bounding_box(0.0, 0.0).unwrap_or_default();
    let box_right = b.bounding_box(0.0, 0.0).unwrap_or_default();

    box_left
        .min()
        .y()
        .partial_cmp(&box_right.min().y())
        .unwrap_or(Ordering::Equal)
}

fn cmp_z(a: &Box<dyn Hitable>, b: &Box<dyn Hitable>) -> Ordering {
    let box_left = a.bounding_box(0.0, 0.0).unwrap_or_default();
    let box_right = b.bounding_box(0.0, 0.0).unwrap_or_default();

    box_left
        .min()
        .z()
        .partial_cmp(&box_right.min().z())
        .unwrap_or(Ordering::Equal)
}
