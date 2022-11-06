use crate::BigNumberError;

const DEFAULT_PRECISION: usize = if cfg!(feature = "precision-1024") {
    1024
} else if cfg!(feature = "precision-512") {
    512
} else {
    256
};

pub struct BigNumber {
    pub value: dashu_float::DBig,
}

impl Clone for BigNumber {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}

impl BigNumber {
    pub fn of(val: &str) -> Result<BigNumber, BigNumberError> {
        let value = match val.strip_prefix("0x") {
            Some(hex) => {
                let value_hex =
                    dashu_float::FBig::<dashu_float::round::mode::HalfAway, 16>::from_str_native(
                        hex,
                    );
                if let Ok(x) = value_hex {
                    dashu_float::DBig::from_str_native(&x.repr().significand().to_string())
                } else {
                    return Err(BigNumberError::ParseError);
                }
            }
            None => dashu_float::DBig::from_str_native(val),
        };

        match value {
            Ok(x) => Ok(BigNumber {
                value: x.with_precision(DEFAULT_PRECISION).value(),
            }),
            Err(_) => Err(BigNumberError::ParseError),
        }
    }

    pub fn of_precision(val: &str, precision: usize) -> Result<BigNumber, BigNumberError> {
        let value = match val.strip_prefix("0x") {
            Some(hex) => {
                let value_hex =
                    dashu_float::FBig::<dashu_float::round::mode::HalfAway, 16>::from_str_native(
                        hex,
                    );
                if let Ok(x) = value_hex {
                    dashu_float::DBig::from_str_native(&x.repr().significand().to_string())
                } else {
                    return Err(BigNumberError::ParseError);
                }
            }
            None => dashu_float::DBig::from_str_native(val),
        };

        match value {
            Ok(x) => Ok(BigNumber {
                value: x.with_precision(precision).value(),
            }),
            Err(_) => Err(BigNumberError::ParseError),
        }
    }

    pub fn is_zero(&self) -> bool {
        self.value.repr().is_zero()
    }

    pub fn to_precision(&self, precision: usize) -> BigNumber {
        BigNumber {
            value: self.value.clone().with_precision(precision).value(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Div;

    use super::*;

    #[test]
    fn test_hex_string() {
        assert_eq!(BigNumber::of("0x42").unwrap().to_string(), "66");
    }

    #[test]
    fn test_hex_string_big() {
        assert_eq!(
            BigNumber::of("0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF")
                .unwrap()
                .to_string(),
            "115792089237316195423570985008687907853269984665640564039457584007913129639935"
        );
    }

    #[test]
    fn test_hex_string_big_div() {
        assert_eq!(
            BigNumber::of("0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF")
                .unwrap()
                .div(BigNumber::from(1e18))
                .to_string(),
            "115792089237316195423570985008687907853269984665640564039457.584007913129639935"
        );
    }
}
