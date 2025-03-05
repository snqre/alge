use crate::common::*;

pub trait Precision: 
    Into<u32> {
    fn f_one_hundred_percent(self) -> MathResult<u128>;
    fn f_rep(self) -> MathResult<u128>;
}

impl Precision for u32 {
    fn f_one_hundred_percent(self) -> MathResult<u128> {
        return 100u128
            .checked_mul(Self::f_rep(self)?)
            .ok_or(MathError::Overflow);
    }    

    fn f_rep(self) -> MathResult<u128> {
        return 10u128
            .checked_pow(self)
            .ok_or(MathError::Overflow);
    }
}