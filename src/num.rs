pub type Num = f64;
pub type Int = i64;

pub const MAX_NUM: Num = std::f64::MAX;
pub const PI: Num = std::f64::consts::PI;

#[inline(always)]
pub fn max(a: Num, b: Num) -> Num {
    if a < b {
        b
    } else {
        a
    }
}

#[inline(always)]
pub fn min(a: Num, b: Num) -> Num {
    if a < b {
        a
    } else {
        b
    }
}

#[inline(always)]
fn min_max(a: Num, b: Num) -> (Num, Num) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }

}
