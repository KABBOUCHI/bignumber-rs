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
        let value = dashu_float::DBig::from_str_native(val);

        match value {
            Ok(x) => Ok(BigNumber {
                value: x.with_precision(DEFAULT_PRECISION).value(),
            }),
            Err(_) => Err(BigNumberError::ParseError),
        }
    }

    pub fn of_precision(val: &str, precision: usize) -> Result<BigNumber, BigNumberError> {
        let value = dashu_float::DBig::from_str_native(val);
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
}
