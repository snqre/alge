#![allow(clippy::needless_return)]
#![allow(clippy::unused_unit)]

mod common;
mod fixed_point_value;
mod precision;

pub use common::*;
pub use fixed_point_value::*;
pub use precision::*;

#[cfg(test)]
mod test {
    use crate::*;

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
    fn f_sub_percentage() -> () {
        1250u128
            .f_sub_percentage(2500u128, 2u32)
            .map(|value| assert!(value == 950u128, "{value}"))
            .unwrap();
        return;
    }

    #[test]
    fn f_cast() -> () {
        250u128
            .f_cast(2u32, 18u32)
            .unwrap()
            .f_cast(18u32, 2u32)
            .map(|value| assert!(value == 250u128, "{value}"))
            .unwrap();
        return;
    }

    #[test]
    fn f_mul() -> () {
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