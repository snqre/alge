pub trait U128Extension<T1, T2>
where 
T1: Into<u128> + From<u128>, 
T2: Into<u8> {
    fn fixed_point_cast(self, old_decimals: T2, new_decimals: T2) -> Option<T1>;
    fn fixed_point_slice_of(self, percentage: T1, decimals: T2) -> Option<T1>;
    fn fixed_point_percentage_gain(self, new_value: T1, decimals: T2) -> Option<T1>;
    fn fixed_point_percentage_loss(self, new_value: T1, decimals: T2) -> Option<T1>;
    fn fixed_point_add_percentage(self, percentage: T1, decimals: T2) -> Option<T1>;
    fn fixed_point_sub_percentage(self, percentage: T1, decimals: T2) -> Option<T1>;
    fn fixed_point_mul(self, rhs: T1, decimals: T2) -> Option<T1>;
    fn fixed_point_div(self, rhs: T1, decimals: T2) -> Option<T1>;
}

impl<T1, T2> U128Extension<T1, T2> for T1
where
T1: Into<u128> + From<u128>,
T2: Into<u8> {
    fn fixed_point_cast(self, old_decimals: T2, new_decimals: T2) -> Option<T1> {
        let value: u128 = self.into();
        let old_decimals: u8 = old_decimals.into();
        let new_decimals: u8 = new_decimals.into();
        if !old_decimals.can_handle_fixed_point_math() {
            return None;
        }
        if !new_decimals.can_handle_fixed_point_math() {
            return None;
        }
        if old_decimals == new_decimals {
            return Some(value.into());
        }
        if value == 0 {
            return Some(value.into());
        }
        let value = value
            .checked_mul(new_decimals.representation()?)?
            .checked_div(old_decimals.representation()?)?;
        return Some(value.into());
    }

    fn fixed_point_slice_of(self, percentage: T1, decimals: T2) -> Option<T1> {
        let value: u128 = self.into();
        let percentage: u128 = percentage.into();
        let decimals: u8 = decimals.into();
        let one_hundred_percent: u128 = decimals.one_hundred_percent()?;
        return value
            .fixed_point_div(one_hundred_percent, decimals)?
            .fixed_point_mul(percentage, decimals)
            .map(|value| value.into());
    }

    fn fixed_point_percentage_gain(self, new_value: T1, decimals: T2) -> Option<T1> {
        let old_value: u128 = self.into();
        let new_value: u128 = new_value.into();
        let decimals: u8 = decimals.into();
        let one_hundred_percent: u128 = decimals.one_hundred_percent()?;
        if new_value <= old_value {
            return Some(0u128.into());
        }
        return new_value
            .checked_sub(old_value)?
            .fixed_point_div(old_value, decimals)?
            .fixed_point_mul(one_hundred_percent, decimals)
            .map(|value| value.into());
    }

    fn fixed_point_percentage_loss(self, new_value: T1, decimals: T2) -> Option<T1> {
        let old_value: u128 = self.into();
        let new_value: u128 = new_value.into();
        let decimals: u8 = decimals.into();
        let one_hundred_percent: u128 = decimals.one_hundred_percent()?;
        if new_value >= old_value {
            return Some(0u128.into());
        }
        return old_value
            .checked_sub(new_value)?
            .fixed_point_div(old_value, decimals)?
            .fixed_point_mul(one_hundred_percent, decimals)
            .map(|value| value.into());
    }

    fn fixed_point_add_percentage(self, percentage: T1, decimals: T2) -> Option<T1> {
        let value: u128 = self.into();
        let percentage: u128 = percentage.into();
        let decimals: u8 = decimals.into();
        let one_hundred_percent: u128 = decimals.one_hundred_percent()?;
        return value
            .fixed_point_div(one_hundred_percent, decimals)?
            .fixed_point_mul(percentage, decimals)?
            .checked_add(value)
            .map(|value| value.into());
    }

    fn fixed_point_sub_percentage(self, percentage: T1, decimals: T2) -> Option<T1> {
        let value: u128 = self.into();
        let percentage: u128 = percentage.into();
        let decimals: u8 = decimals.into();
        let one_hundred_percent: u128 = decimals.one_hundred_percent()?;
        let result: u128 = value
            .fixed_point_div(one_hundred_percent, decimals)?
            .fixed_point_mul(percentage, decimals)?;
        return value
            .checked_sub(result)
            .map(|value| value.into());
    }

    fn fixed_point_mul(self, rhs: T1, decimals: T2) -> Option<T1> {
        let value0: u128 = self.into();
        let value1: u128 = rhs.into();
        let decimals: u8 = decimals.into();
        return value0
            .checked_mul(value1)?
            .checked_div(decimals.representation()?)
            .map(|value| value.into());
    }

    fn fixed_point_div(self, rhs: T1, decimals: T2) -> Option<T1> {
        let value0: u128 = self.into();
        let value1: u128 = rhs.into();
        let decimals: u8 = decimals.into();
        return value0
            .checked_mul(decimals.representation()?)?
            .checked_div(value1)
            .map(|value| value.into());
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

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test2() -> () {
        500u32.fixed
    }

    #[test]
    fn test_u8_can_handle_fixed_point_math() -> () {
        assert!(18u8.can_handle_fixed_point_math());
        assert!(!1u8.can_handle_fixed_point_math());
        return;
    }
}