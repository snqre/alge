#![allow(clippy::needless_return)]
#![allow(clippy::unused_unit)]


// # Math

pub type MathResult<T1> = Result<T1, MathError>;

#[derive(Debug)]
pub enum MathError {
    Overflow,
    Underflow,
    DivByZero,
    UnsupportedDecimals
}

// # U32 Extension

pub trait U32Extension : Into<u32> {
    fn f_one_hundred_percent(self) -> MathResult<u128>;
    fn f_rep(self) -> MathResult<u128>;
}

impl U32Extension for u32 {
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


// # U128 Extension

pub trait U128Extension : Into<u128> {
    fn f_slice(self, percentage: u128, decimals: u32) -> MathResult<u128>;
    fn f_percentage_gain(self, new_value: u128, decimals: u32) -> MathResult<u128>;
    fn f_percentage_loss(self, new_value: u128, decimals: u32) -> MathResult<u128>;
    fn f_add_percentage(self, percentage: u128, decimals: u32) -> MathResult<u128>;
    fn f_sub_percentage(self, percentage: u128, decimals: u32) -> MathResult<u128>;
    fn f_cast(self, old_decimals: u32, new_decimals: u32) -> MathResult<u128>;
    fn f_mul(self, rhs: u128, decimals: u32) -> MathResult<u128>;
    fn f_div(self, rhs: u128, decimals: u32) -> MathResult<u128>;
}

impl U128Extension for u128 {
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

    fn f_cast(self, old_decimals: u32, new_decimals: u32) -> MathResult<u128> {
        if old_decimals < 2
        || new_decimals < 2 {
            return Err(MathError::UnsupportedDecimals);
        }
        if self == 0
        || old_decimals == new_decimals {
            return Ok(self);
        }
        let old_rep: u128 = old_decimals.f_rep()?;
        let new_rep: u128 = new_decimals.f_rep()?;
        return self
            .checked_mul(new_rep)
            .ok_or(MathError::Overflow)?
            .checked_div(old_rep)
            .ok_or(MathError::DivByZero);
    }

    fn f_mul(self, rhs: u128, decimals: u32) -> MathResult<u128> {
        let rep: u128 = decimals.f_rep()?;
        return self
            .checked_mul(rhs)
            .ok_or(MathError::Overflow)?
            .checked_div(rep)
            .ok_or(MathError::DivByZero);
    }
    
    fn f_div(self, rhs: u128, decimals: u32) -> MathResult<u128> {
        let rep: u128 = decimals.f_rep()?;
        return self
            .checked_mul(rep)
            .ok_or(MathError::Overflow)?
            .checked_div(rhs)
            .ok_or(MathError::DivByZero);
    }
}

#[cfg(test)]
mod test {
    use crate::{U128Extension, U32Extension};

    #[test]
    fn f_one_hundred_percent() -> () {
        2u32
            .f_one_hundred_percent()
            .map(|value| assert!(value == 10000u128, "{value}"))
            .unwrap();
        return;
    }

    #[test]
    fn f_rep() -> () {
        2u32
            .f_rep()
            .map(|value| assert!(value == 100u128, "{value}"))
            .unwrap();
        return;
    }

    #[test]
    fn f_slice() -> () {
        6500u128
            .f_slice(2500u128, 2u32)
            .map(|value| assert!(value == 1625u128, "{value}"))
            .unwrap();
        return;
    }

    #[test]
    fn f_percentage_gain() -> () {
        2500u128
            .f_percentage_gain(6500u128, 2u32)
            .map(|value| assert!(value == 16000u128, "{value}"))
            .unwrap();
        return;
    }

    #[test]
    fn f_percentage_loss() -> () {
        6500u128
            .f_percentage_loss(2500u128, 2u32)
            .map(|value| assert!(value == 6100u128, "{value}"))
            .unwrap();
        return;
    }

    #[test]
    fn f_add_percentage() -> () {
        500u128
            .f_add_percentage(2500u128, 2u32)
            .map(|value| assert!(value == 625u128, "{value}"))
            .unwrap();
        return;
    }

    #[test]
    fn f_sub_percentage() {
        1250u128
            .f_sub_percentage(2500u128, 2u32)
            .map(|value| assert!(value == 950u128, "{value}"))
            .unwrap();
        return;
    }

    #[test]
    fn f_cast() {
        250u128
            .f_cast(2u32, 18u32)
            .unwrap()
            .f_cast(18u32, 2u32)
            .map(|value| assert!(value == 250u128, "{value}"))
            .unwrap();
        return;
    }

    #[test]
    fn f_mul() {
        4550u128
            .f_mul(50u128, 2u32)
            .map(|value| assert!(value == 2275u128, "{value}"))
            .unwrap();
        return;
    }

    #[test]
    fn f_div() -> () {
        4550u128
            .f_div(5000u128, 2u32)
            .map(|value| assert!(value == 91u128, "{value}"))
            .unwrap();
        return;
    }
}