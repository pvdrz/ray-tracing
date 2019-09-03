use crate::num::*;

macro_rules! impl_op {
    ($trait:tt, $method:tt) => {
        use std::ops::$trait;

        impl $trait for Vec3 {
            type Output = Vec3;
            fn $method(self, other: Vec3) -> Self::Output {
                Vec3::new(
                    self.x.$method(other.x),
                    self.y.$method(other.y),
                    self.z.$method(other.z),
                )
            }
        }

        impl $trait<Num> for Vec3 {
            type Output = Vec3;

            fn $method(self, other: Num) -> Self::Output {
                Vec3::new(
                    self.x.$method(other),
                    self.y.$method(other),
                    self.z.$method(other),
                )
            }
        }

        impl $trait<Vec3> for Num {
            type Output = Vec3;

            fn $method(self, other: Vec3) -> Self::Output {
                Vec3::new(
                    self.$method(other.x),
                    self.$method(other.y),
                    self.$method(other.z),
                )
            }
        }
    };
}
macro_rules! impl_op_assign {
    ($trait:tt, $method:tt) => {
        use std::ops::$trait;

        impl $trait for Vec3 {
            fn $method(&mut self, other: Vec3) {
                self.x.$method(other.x);
                self.y.$method(other.y);
                self.z.$method(other.z);
            }
        }

        impl $trait<Num> for Vec3 {
            fn $method(&mut self, other: Num) {
                self.x.$method(other);
                self.y.$method(other);
                self.z.$method(other);
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

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    x: Num,
    y: Num,
    z: Num,
}

impl Vec3 {
    pub fn new(x: Num, y: Num, z: Num) -> Self {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn from_scalar(x: Num) -> Self {
        Self::new(x, x, x)
    }

    pub fn dot(self, other: Self) -> Num {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn len(&self) -> Num {
        self.dot(*self).sqrt()
    }

    pub fn cross(self, other: Self) -> Self {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn unit(self) -> Self {
        self / self.len()
    }

    pub fn x(&self) -> Num {
        self.x
    }

    pub fn y(&self) -> Num {
        self.y
    }

    pub fn z(&self) -> Num {
        self.z
    }

    pub fn r(&self) -> Num {
        self.x
    }

    pub fn g(&self) -> Num {
        self.y
    }

    pub fn b(&self) -> Num {
        self.z
    }

    pub fn min(&self, other: &Self) -> Self {
        Self::new(
            min(self.x, other.x),
            min(self.y, other.y),
            min(self.z, other.z),
        )
    }

    pub fn max(&self, other: &Self) -> Self {
        Self::new(
            max(self.x, other.x),
            max(self.y, other.y),
            max(self.z, other.z),
        )
    }

    pub fn min_max(&self, other: &Self) -> (Self, Self) {
        let (x_min, x_max) = if self.x > other.x {
            (other.x, self.x)
        } else {
            (self.x, other.x)
        };
        let (y_min, y_max) = if self.y > other.y {
            (other.y, self.y)
        } else {
            (self.y, other.y)
        };
        let (z_min, z_max) = if self.z > other.z {
            (other.z, self.z)
        } else {
            (self.z, other.z)
        };
        (
            Vec3::new(x_min, y_min, z_min),
            Vec3::new(x_max, y_max, z_max),
        )
    }
}
