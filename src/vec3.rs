use crate::Num;

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
                    self.r,
                    self.g,
                    self.b,
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
                    self.r,
                    self.g,
                    self.b,
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
    pub x: Num,
    pub y: Num,
    pub z: Num,
    pub r: Num,
    pub g: Num,
    pub b: Num,
}

impl Vec3 {
    pub fn new(x: Num, y: Num, z: Num, r: Num, g: Num, b: Num) -> Self {
        Vec3 { x, y, z, r, g, b }
    }

    pub fn dot(self, other: Self) -> Num {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn len(&self) -> Num {
        self.dot(*self).sqrt()
    }

    pub fn cross(self, other: Self) -> Self {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }

    pub fn unit(self) -> Self {
        self / self.len()
    }
}
