#![doc = include_str!("../README.md")]
mod add;
mod bignumber;
mod convert;
mod div;
mod exp;
mod fmt;
mod macros;
mod mul;
mod primitive_types;
mod sub;

pub type BigNumber = bignumber::BigNumber;

#[derive(Debug)]
pub enum BigNumberError {
    InvalidDigit,
}
