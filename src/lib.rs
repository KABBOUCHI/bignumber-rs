#![doc = include_str!("../README.md")]
mod add;
mod bignumber;
mod convert;
mod div;
mod ethereum_types;
mod exp;
mod fmt;
mod macros;
mod mul;
mod sub;

pub type BigNumber = bignumber::BigNumber;

#[derive(Debug)]
pub enum BigNumberError {
    InvalidDigit,
}
