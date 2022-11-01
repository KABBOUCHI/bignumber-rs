use crate::bignumber::BigNumber;

impl BigNumber {
    pub fn pow(&self, exp: &BigNumber) -> BigNumber {
        BigNumber {
            value: self.value.powf(&exp.value),
        }
    }

    pub fn powi(&self, exp: i32) -> BigNumber {
        BigNumber {
            value: self.value.powi(exp.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::to_bn_safe;

    #[test]
    fn test_powi() {
        assert_eq!(to_bn_safe!("2").powi(12).to_string(), "4096",);
    }
}
