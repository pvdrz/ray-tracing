use crate::vec3::Vec3;
use crate::num::*;

#[derive(Clone)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn zero() -> Self {
        Ray::new(Vec3::zero(), Vec3::zero())
    }

    pub fn point_at(&self, t: Num) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }
}
