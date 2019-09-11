use packed_simd::*;

pub type Num = f64;
pub type Int = i64;

pub type Numx4 = f64x4;
pub type Intx4 = i64x4;

pub const MAX_NUM: Num = std::f64::MAX;
pub const PI: Num = std::f64::consts::PI;

pub fn max(a: Num, b: Num) -> Num {
    if a < b {
        b
    } else {
        a
    }
}

pub fn min(a: Num, b: Num) -> Num {
    if a < b {
        a
    } else {
        b
    }
}
