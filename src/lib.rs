use std::{ops::{Add, Mul, RangeBounds}, process::Output};

pub trait U128Extension<T1, T2>
where 
T1: Into<u128>, 
T2: Into<u8> {
    fn fixed_point_cast(self, old_decimals: T2, new_decimals: T2) -> T1;
    fn fixed_point_slice_of(self, percentage: T1, decimals: T2) -> T1;
    fn fixed_point_percentage_gain(self, new_value: T1, decimals: T2) -> T1;
    fn fixed_point_percentage_loss(self, new_value: T1, decimals: T2) -> T1;
    fn fixed_point_add_percentage(self, percentage: T1, decimals: T2) -> T1;
    fn fixed_point_sub_percentage(self, percentage: T1, decimals: T2) -> T1;
    fn fixed_point_mul(self, rhs: T1) -> T1;
    fn fixed_point_div(self, rhs: T1) -> T1;
    fn checked_fixed_point_cast(self, old_decimals: T2, new_decimals: T2) -> Option<T1>;
    fn checked_fixed_point_slice_of(self, percentage: T1, decimals: T2) -> Option<T1>;
    fn checked_fixed_point_percentage_gain(self, new_value: T1, decimals: T2) -> Option<T1>;
    fn checked_fixed_point_percentage_loss(self, new_value: T1, decimals: T2) -> Option<T1>;
    fn checked_fixed_point_add_percentage(self, percentage: T1, decimals: T2) -> Option<T1>;
    fn checked_fixed_point_sub_percentage(self, percentage: T1, decimals: T2) -> Option<T1>;
    fn checked_fixed_point_mul(self, rhs: T1) -> Option<T1>;
    fn checked_fixed_point_div(self, rhs: T1) -> Option<T1>;
}

impl<T1, T2> U128Extension<T1, T2> for T1
where
T1: Into<u128>,
T2: Into<u8> {
    fn checked_fixed_point_cast(self, old_decimals: T2, new_decimals: T2) -> Option<u128> {
        let value: u128 = self.into();
        let old_decimals: u8 = old_decimals.into();
        let new_decimals: u8 = new_decimals.into();
        if !old_decimals.representation() {
            return None;
        }
        if !new_decimals.representation() {
            return None;
        }
        if old_decimals == new_decimals {
            return Some(value);
        }
        if value == 0 {
            return Some(value);
        }
        let value = value
            .checked_mul(new_decimals.representation()?)?
            .checked_div(old_decimals.representation()?)?;
        return Some(value);
    }
}



pub trait U8Extension {
    fn can_handle_fixed_point_math(self) -> bool;
    fn one_hundred_percent(self) -> Option<u128>;
    fn representation(self) -> Option<u128>;
}

impl U8Extension for u8 {
    fn can_handle_fixed_point_math(self) -> bool {
        return self >= 2u8;
    }

    fn one_hundred_percent(self) -> Option<u128> {
        return 100u128.checked_mul(self.representation()?);
    }

    fn representation(self) -> Option<u128> {
        return 10u128.checked_pow(self.into());
    }
}



pub fn slice_of(value: u128, percentage: u128, decimals: u8) -> Option<u128> {
    return mul(div(value, one_hundred_percent_from(decimals)?, decimals)?, percentage, decimals);
}

pub fn percentage_gain(old_value: u128, new_value: u128, decimals: u8) -> Option<u128> {
    if new_value <= old_value {
        return Some(0);
    }
    return mul(div(sub(new_value, old_value)?, old_value, decimals)?, one_hundred_percent_from(decimals)?, decimals);
}

pub fn percentage_loss(old_value: u128, new_value: u128, decimals: u8) -> Option<u128> {
    if new_value >= old_value {
        return Some(0);
    }
    let one_hundred_percent: u128 = one_hundred_percent_from(decimals)?;
    let result: u128 = sub(old_value, new_value)?;
    let result: u128 = div(result, old_value, decimals)?;
    let result: u128 = mul(result, one_hundred_percent, decimals)?;
    return Some(result);
}

pub fn add_percentage(value: u128, percentage: u128, decimals: u8) -> Option<u128> {
    let scale: u128 = one_hundred_percent_from(decimals)?;
    let result: u128 = div(value, scale, decimals)?;
    let result: u128 = mul(result, percentage, decimals)?;
    let result: u128 = add(result, value)?;
    return Some(result);
}

pub fn sub_percentage(value: u128, percentage: u128, decimals: u8) -> Option<u128> {
    let scale: u128 = one_hundred_percent_from(decimals)?;
    let result: u128 = div(value, scale, decimals)?;
    let result: u128 = mul(result, percentage, decimals)?;
    let result: u128 = sub(value, result)?;
    return Some(result);
}

pub fn add(x: u128, y: u128) -> Option<u128> {
    return x.checked_add(y);
}

pub fn sub(x: u128, y: u128) -> Option<u128> {
    return x.checked_sub(y);
}

pub fn mul(x: u128, y: u128, decimals: u8) -> Option<u128> {
   return x
        .checked_mul(y)?
        .checked_div(representation_of(decimals)?);
}

pub fn div(x: u128, y: u128, decimals: u8) -> Option<u128> {
    return x
        .checked_mul(representation_of(decimals)?)?
        .checked_div(y);
}

pub fn to_precision(value: u128, old_decimals: u8, new_decimals: u8) -> Option<u128> {
    if !can_handle_fixed_point_math(old_decimals) {
        return None;
    }
    if !can_handle_fixed_point_math(new_decimals) {
        return None;
    }
    if old_decimals == new_decimals {
        return Some(value);
    }
    if value == 0 {
        return Some(value);
    }
    let result = value
        .checked_mul(representation_of(new_decimals)?)?
        .checked_div(representation_of(old_decimals)?)?;
    return Some(result);
}
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_slice_of() -> () {
        slice_of(5000000000u128, 2500000000u128, 8u8)
            .map(|value| {
                assert!(value == 1250000000u128, "{value}");
                return;
            })
            .unwrap();
        return;
    }

    #[test]
    fn test_percentage_gain() -> () {
        let old_value: u128 = 5000000000u128;
        let new_value: u128 = 9000000000u128;
        percentage_gain(old_value, new_value, 8u8)
            .map(|value| {
                assert!(value == 8000000000u128, "{value}");
                return;
            })
            .unwrap();
        return;
    }

    #[test]
    fn test_percentage_loss() -> () {
        let x: u128 = to_precision(5000, 2, 18).unwrap();
        let y: u128 = to_precision(2500, 2, 18).unwrap();
        percentage_loss(x, y, 18)
            .map(|value| {
                let value: u128 = to_precision(value, 18, 2).unwrap();
                assert!(value == 5000, "{value}");
            })
            .unwrap();
    }

    #[test]
    fn test_add_percentage() -> () {
        let x: u128 = to_precision(5000u128, 2u8, 18u8).unwrap();
        let y: u128 = to_precision(5000u128, 2u8, 18u8).unwrap();
        
        5000u128
            .checked_add_percentage(percentage, decimals)
            .unwrap();

        add_percentage(x, y, 18)
            .map(|value| {
                let value: u128 = to_precision(value, 18u8, 2u8).unwrap();
                assert!(value == 7500u128, "{value}");
                return;
            })
            .unwrap();
        return;
    }

    #[test]
    fn test_mul() -> () {
        mul(60u128, 3500u128, 2u8)
            .map(|value| {
                assert!(value == 2100u128);
                return;
            })
            .unwrap();
        return;
    }

    #[test]
    fn test_div() -> () {
        let x = to_precision(550u128, 2, 18).unwrap_or(0);
        let y = to_precision(5000, 2, 18).unwrap_or(0);
        sub_percentage(x, y, 18u8) // 5.50, 0.15
            .map(|value| {
                let z = to_precision(value, 18, 2).unwrap_or(0);
                assert!(z == 275u128, "{z}"); // 36.66
                return;
            })
            .unwrap();
        return;
    }

    #[test]
    fn test_to_precision() -> () {
        to_precision(200u128, 2u8, 18u8)
            .map(|value| {
                assert!(value == 2000000000000000000u128);
                return;
            })
            .unwrap();
        return;
    }

    #[test]
    fn test_one_hundred_percent_from() -> () {
        one_hundred_percent_from(2u8)
            .map(|one_hundred_percent| {
                assert!(one_hundred_percent == 10000u128);
                return;
            })
            .unwrap();
        return;
    }

    #[test]
    fn test_u8_representation() -> () {
        2u8
            .representation()
            .map(|value| {
                assert!(value == 100u128, "{value}");
                return;
            })
            .expect(msg);
    }

    #[test]
    fn test_u8_can_handle_fixed_point_math() -> () {
        assert!(18u8.can_handle_fixed_point_math());
        assert!(!1u8.can_handle_fixed_point_math());
        return;
    }
}