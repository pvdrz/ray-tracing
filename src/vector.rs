use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub type Float = f32;

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    e0: Float,
    e1: Float,
    e2: Float,
}

impl Vector {
    pub fn new(e0: Float, e1: Float, e2: Float) -> Self {
        Vector { e0, e1, e2 }
    }

    #[inline(always)]
    pub fn x(&self) -> Float {
        self.e0
    }

    #[inline(always)]
    pub fn y(&self) -> Float {
        self.e1
    }

    #[inline(always)]
    pub fn z(&self) -> Float {
        self.e2
    }

    #[inline(always)]
    pub fn r(&self) -> Float {
        self.e0
    }

    #[inline(always)]
    pub fn g(&self) -> Float {
        self.e1
    }

    #[inline(always)]
    pub fn b(&self) -> Float {
        self.e2
    }

    #[inline(always)]
    pub fn len(&self) -> Float {
        self.len_squared().sqrt()
    }

    #[inline(always)]
    pub fn len_squared(&self) -> Float {
        self.dot(*self)
    }

    #[inline(always)]
    pub fn dot(self, other: Self) -> Float {
        self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2
    }

    #[inline(always)]
    pub fn cross(self, other: Self) -> Self {
        Vector::new(
            self.e1 * other.e2 - self.e2 * other.e1,
            self.e2 * other.e0 - self.e0 * other.e2,
            self.e0 * other.e1 - self.e1 * other.e0,
        )
    }

    #[inline(always)]
    pub fn make_unit_vector(&mut self) {
        *self /= 1.0 / self.len();
    }

    #[inline(always)]
    pub fn unit_vector(self) -> Self {
        self / self.len()
    }
}

impl Neg for Vector {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Vector {
            e0: -self.e0,
            e1: -self.e1,
            e2: -self.e2,
        }
    }
}

impl AddAssign for Vector {
    #[inline(always)]
    fn add_assign(&mut self, other: Self) {
        self.e0 += other.e0;
        self.e1 += other.e1;
        self.e2 += other.e2;
    }
}

impl Add for Vector {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self.e0 + rhs.e0, self.e1 + rhs.e1, self.e2 + rhs.e2)
    }
}

impl SubAssign for Vector {
    #[inline(always)]
    fn sub_assign(&mut self, other: Self) {
        self.e0 -= other.e0;
        self.e1 -= other.e1;
        self.e2 -= other.e2;
    }
}

impl Sub for Vector {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.e0 - rhs.e0, self.e1 - rhs.e1, self.e2 - rhs.e2)
    }
}

impl MulAssign for Vector {
    #[inline(always)]
    fn mul_assign(&mut self, other: Self) {
        self.e0 *= other.e0;
        self.e1 *= other.e1;
        self.e2 *= other.e2;
    }
}

impl Mul for Vector {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        Vector::new(self.e0 * rhs.e0, self.e1 * rhs.e1, self.e2 * rhs.e2)
    }
}

impl DivAssign for Vector {
    #[inline(always)]
    fn div_assign(&mut self, other: Self) {
        self.e0 /= other.e0;
        self.e1 /= other.e1;
        self.e2 /= other.e2;
    }
}

impl Div for Vector {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        Vector::new(self.e0 / rhs.e0, self.e1 / rhs.e1, self.e2 / rhs.e2)
    }
}

impl MulAssign<Float> for Vector {
    #[inline(always)]
    fn mul_assign(&mut self, other: Float) {
        self.e0 *= other;
        self.e1 *= other;
        self.e2 *= other;
    }
}

impl Mul<Float> for Vector {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Float) -> Self::Output {
        Vector::new(self.e0 * rhs, self.e1 * rhs, self.e2 * rhs)
    }
}

impl Mul<Vector> for Float {
    type Output = Vector;

    #[inline(always)]
    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::new(self * rhs.e0, self * rhs.e1, self * rhs.e2)
    }
}

impl DivAssign<Float> for Vector {
    #[inline(always)]
    fn div_assign(&mut self, other: Float) {
        let k = 1.0 / other;
        self.e0 *= k;
        self.e1 *= k;
        self.e2 *= k;
    }
}

impl Div<Float> for Vector {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Float) -> Self::Output {
        Vector::new(self.e0 / rhs, self.e1 / rhs, self.e2 / rhs)
    }
}
