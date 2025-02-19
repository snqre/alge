pub fn slice_of(value: u128, percentage: u128, decimals: u8) -> Option<u128> {
    return mul(div(value, one_hundred_percent_from(decimals)?, decimals)?, percentage, decimals);
}

pub fn percentage_gain(old_value: u128, new_value: u128, decimals: u8) -> Option<u128> {
    if old_value <= new_value {
        return Some(0);
    }
    return mul(div(sub(new_value, old_value)?, old_value, decimals)?, one_hundred_percent_from(decimals)?, decimals);
}

pub fn percentage_loss(old_value: u128, new_value: u128, decimals: u8) -> Option<u128> {
    if new_value >= old_value {
        return Some(0);
    }
    return mul(div(sub(old_value, new_value)?, old_value, decimals)?, one_hundred_percent_from(decimals)?, decimals);
}

pub fn add_percentage(value: u128, percentage: u128, decimals: u8) -> Option<u128> {
    return add(mul(div(value, one_hundred_percent_from(decimals)?, decimals)?, percentage, decimals)?, value);
}

pub fn sub_percentage(value: u128, percentage: u128, decimals: u8) -> Option<u128> {
    return sub(mul(div(value, one_hundred_percent_from(decimals)?, decimals)?, percentage, decimals)?, value);
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

pub fn one_hundred_percent_from(decimals: u8) -> Option<u128> {
    return 100u128.checked_mul(representation_of(decimals)?);
}

pub fn representation_of(decimals: u8) -> Option<u128> {
    let decimals: u32 = decimals.into();
    return 10u128.checked_pow(decimals);
}

pub fn can_handle_fixed_point_math(decimals: u8) -> bool {
    return decimals >= 2u8;
}

#[cfg(test)]
mod tests {
    use crate::*;

    

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
    fn test_representation_of() -> () {
        representation_of(2u8)
            .map(|representation| {
                assert!(representation == 100u128);
                return;
            })
            .unwrap();
        return;
    }

    #[test]
    fn test_can_handle_fixed_point_math() -> () {
        assert!(can_handle_fixed_point_math(18u8));
        assert!(!can_handle_fixed_point_math(1u8));
        return;
    }
}