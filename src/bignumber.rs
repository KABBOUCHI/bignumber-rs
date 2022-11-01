use crate::errors::BigNumberError;

pub struct BigNumber {
    pub value: dashu_float::DBig,
}

impl BigNumber {
    pub fn of(val: &str) -> Result<BigNumber, BigNumberError> {
        let value = dashu_float::DBig::from_str_native(val);

        match value {
            Ok(x) => Ok(BigNumber {
                value: x.with_precision(512).value(),
            }),
            Err(_) => Err(BigNumberError::InvalidDigit),
        }
    }

    pub fn of_precision(val: &str, precision: usize) -> Result<BigNumber, BigNumberError> {
        let value = dashu_float::DBig::from_str_native(val);
        match value {
            Ok(x) => Ok(BigNumber {
                value: x.with_precision(precision).value(),
            }),
            Err(_) => Err(BigNumberError::InvalidDigit),
        }
    }
}
