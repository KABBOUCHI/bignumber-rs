use std::str::FromStr;

use crate::{BigNumber, BigNumberError};

impl From<usize> for BigNumber {
    #[inline]
    fn from(n: usize) -> Self {
        BigNumber::of(n.to_string().as_str()).unwrap()
    }
}

impl FromStr for BigNumber {
    type Err = BigNumberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BigNumber::of(s)
    }
}
