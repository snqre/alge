use crate::common::*;
use crate::precision::*;

pub trait FixedPointValue:
    Into<u128> {
    fn f_slice(self, percentage: u128, decimals: u32) -> MathResult<u128>;
    fn f_percentage_gain(self, new_value: u128, decimals: u32) -> MathResult<u128>;
    fn f_percentage_loss(self, new_value: u128, decimals: u32) -> MathResult<u128>;
    fn f_add_percentage(self, percentage: u128, decimals: u32) -> MathResult<u128>;
    fn f_sub_percentage(self, percentage: u128, decimals: u32) -> MathResult<u128>;
    fn f_cast(self, old_precision: u32, new_precision: u32) -> MathResult<u128>;
    fn f_mul(self, rhs: u128, decimals: u32) -> MathResult<u128>;
    fn f_div(self, rhs: u128, decimals: u32) -> MathResult<u128>;
}

impl FixedPointValue for u128 {
    fn f_slice(self, percentage: u128, decimals: u32) -> MathResult<u128> {
        return self
            .f_div(
                10u128
                    .checked_pow(decimals)
                    .ok_or(MathError::Overflow)?
                    .checked_mul(100u128)
                    .ok_or(MathError::Overflow)?,
                decimals
            )?
            .f_mul(percentage, decimals);
    }
    
    fn f_percentage_gain(self, new_value: u128, decimals: u32) -> MathResult<u128> {
        let old_value: u128 = self;
        let new_value: u128 = new_value;
        let one_hundred_percent: u128 = decimals.f_one_hundred_percent()?;
        if new_value <= old_value {
            return Ok(0u128);
        }
        return new_value
            .checked_sub(old_value)
            .ok_or(MathError::Underflow)?
            .f_div(old_value, decimals)?
            .f_mul(one_hundred_percent, decimals);
    }

    fn f_percentage_loss(self, new_value: u128, decimals: u32) -> MathResult<u128> {
        let old_value: u128 = self;
        let new_value: u128 = new_value;
        let one_hundred_percent: u128 = decimals.f_one_hundred_percent()?;
        if new_value >= old_value {
            return Ok(0u128);
        }
        return old_value
            .checked_sub(new_value)
            .ok_or(MathError::Underflow)?
            .f_div(old_value, decimals)?
            .f_mul(one_hundred_percent, decimals);
    }

    fn f_add_percentage(self, percentage: u128, decimals: u32) -> MathResult<u128> {
        let one_hundred_percent: u128 = decimals.f_one_hundred_percent()?;
        return self
            .f_div(one_hundred_percent, decimals)?
            .f_mul(percentage, decimals)?
            .checked_add(self)
            .ok_or(MathError::Overflow);
    }

    fn f_sub_percentage(self, percentage: u128, decimals: u32) -> MathResult<u128> {
        let one_hundred_percent: u128 = decimals.f_one_hundred_percent()?;
        let amount_less: u128 = self
            .f_div(one_hundred_percent, decimals)?
            .f_mul(percentage, decimals)?;
        return self
            .checked_sub(amount_less)
            .ok_or(MathError::Underflow);
    }

    fn f_cast(self, old_precision: u32, new_precision: u32) -> MathResult<u128> {
        if old_precision < 2
        || new_precision < 2 {
            return Err(MathError::IncompatiblePrecision);
        }
        if self == 0
        || old_precision == new_precision {
            return Ok(self);
        }
        let old_rep: u128 = old_precision.f_rep()?;
        let new_rep: u128 = new_precision.f_rep()?;
        return self
            .checked_mul(new_rep)
            .ok_or(MathError::Overflow)?
            .checked_div(old_rep)
            .ok_or(MathError::DivisionByZero);
    }

    fn f_mul(self, rhs: u128, decimals: u32) -> MathResult<u128> {
        let rep: u128 = decimals.f_rep()?;
        return self
            .checked_mul(rhs)
            .ok_or(MathError::Overflow)?
            .checked_div(rep)
            .ok_or(MathError::DivisionByZero);
    }
    
    fn f_div(self, rhs: u128, decimals: u32) -> MathResult<u128> {
        let rep: u128 = decimals.f_rep()?;
        return self
            .checked_mul(rep)
            .ok_or(MathError::Overflow)?
            .checked_div(rhs)
            .ok_or(MathError::DivisionByZero);
    }
}