use crate::num::*;
use crate::vec3::Vec3;

#[derive(Clone, Default)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn point_at(&self, t: Num) -> Vec3 {
        self.origin + t * self.direction
    }

    #[inline(always)]
    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    #[inline(always)]
    pub fn direction(&self) -> Vec3 {
        self.direction
    }
}
