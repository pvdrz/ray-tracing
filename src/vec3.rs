use crate::num::*;
use packed_simd::*;

macro_rules! impl_op {
    ($trait:tt, $method:tt) => {
        use std::ops::$trait;

        impl $trait for Vec3 {
            type Output = Vec3;
            fn $method(self, other: Vec3) -> Self::Output {
                Vec3 {
                    inner: self.inner.$method(other.inner),
                }
            }
        }

        impl $trait<Num> for Vec3 {
            type Output = Vec3;

            fn $method(self, other: Num) -> Self::Output {
                Vec3 {
                    inner: self.inner.$method(other),
                }
            }
        }

        impl $trait<Vec3> for Num {
            type Output = Vec3;

            fn $method(self, other: Vec3) -> Self::Output {
                Vec3 {
                    inner: self.$method(other.inner),
                }
            }
        }
    };
}
macro_rules! impl_op_assign {
    ($trait:tt, $method:tt) => {
        use std::ops::$trait;

        impl $trait for Vec3 {
            fn $method(&mut self, other: Vec3) {
                self.inner.$method(other.inner);
            }
        }

        impl $trait<Num> for Vec3 {
            fn $method(&mut self, other: Num) {
                self.inner.$method(other);
            }
        }
    };
}

impl_op!(Add, add);
impl_op!(Sub, sub);
impl_op!(Mul, mul);
impl_op!(Div, div);
impl_op_assign!(AddAssign, add_assign);
impl_op_assign!(SubAssign, sub_assign);
impl_op_assign!(MulAssign, mul_assign);
impl_op_assign!(DivAssign, div_assign);

#[derive(Debug, Copy, Clone, Default)]
pub struct Vec3 {
    inner: Numx4,
}

impl Vec3 {
    pub fn new(x: Num, y: Num, z: Num) -> Self {
        Vec3 {
            inner: Numx4::new(x, y, z, 0.0),
        }
    }

    pub fn from_scalar(x: Num) -> Self {
        Self::new(x, x, x)
    }

    pub fn dot(self, other: Self) -> Num {
        (self.inner * other.inner).replace(3, 0.0).sum()
    }

    pub fn len(self) -> Num {
        self.dot(self).sqrt()
    }

    pub fn sqrt(self) -> Self {
        Vec3 {
            inner: self.inner.sqrt(),
        }
    }

    pub fn cross(self, other: Self) -> Self {
        let a = self.inner;
        let b = other.inner;
        let a1: Numx4 = shuffle!(a, [1, 2, 0, 3]);
        let a2: Numx4 = shuffle!(a, [2, 0, 1, 3]);
        let b1: Numx4 = shuffle!(b, [2, 0, 1, 3]);
        let b2: Numx4 = shuffle!(b, [1, 2, 0, 3]);

        Vec3 {
            inner: a1 * b1 - a2 * b2,
        }
    }

    pub fn unit(self) -> Self {
        self / self.len()
    }

    pub fn x(&self) -> Num {
        self.inner.extract(0)
    }

    pub fn y(&self) -> Num {
        self.inner.extract(1)
    }

    pub fn z(&self) -> Num {
        self.inner.extract(2)
    }

    pub fn r(&self) -> Num {
        self.inner.extract(0)
    }

    pub fn g(&self) -> Num {
        self.inner.extract(1)
    }

    pub fn b(&self) -> Num {
        self.inner.extract(2)
    }

    pub fn min(&self, other: &Self) -> Self {
        Vec3 {
            inner: self.inner.lt(other.inner).select(self.inner, other.inner),
        }
    }

    pub fn max(&self, other: &Self) -> Self {
        Vec3 {
            inner: self.inner.gt(other.inner).select(self.inner, other.inner),
        }
    }

    pub fn min_max(&self, other: &Self) -> (Self, Self) {
        let mask = self.inner.lt(other.inner);
        (
            Vec3 {
                inner: mask.select(self.inner, other.inner),
            },
            Vec3 {
                inner: mask.select(other.inner, self.inner),
            },
        )
    }
}
