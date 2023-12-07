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
mod serde;
mod sub;

pub type BigNumber = bignumber::BigNumber;

#[derive(Debug)]
pub enum BigNumberError {
    ParseError,
}

impl std::error::Error for BigNumberError {}

impl std::fmt::Display for BigNumberError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            BigNumberError::ParseError => "failed to parse".fmt(f),
        }
    }
}
