use core::ops::Sub;

use crate::BigNumber;

impl Sub for BigNumber {
    type Output = BigNumber;

    fn sub(self, rhs: BigNumber) -> Self::Output {
        BigNumber {
            value: self.value.sub(rhs.value),
        }
    }
}

impl Sub<&BigNumber> for BigNumber {
    type Output = BigNumber;

    fn sub(self, rhs: &BigNumber) -> Self::Output {
        BigNumber {
            value: self.value.sub(rhs.value.clone()),
        }
    }
}

impl Sub for &BigNumber {
    type Output = BigNumber;

    fn sub(self, rhs: &BigNumber) -> Self::Output {
        BigNumber {
            value: self.value.clone().sub(rhs.value.clone()),
        }
    }
}

impl Sub<BigNumber> for &BigNumber {
    type Output = BigNumber;

    fn sub(self, rhs: BigNumber) -> Self::Output {
        BigNumber {
            value: self.value.clone().sub(rhs.value),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::to_bn_safe;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(to_bn_safe!("1").sub(to_bn_safe!("3")).to_string(), "-2");
    }

    #[test]
    fn test_b() {
        assert_eq!(
            to_bn_safe!("0.3").sub(to_bn_safe!("0.2")).to_string(),
            "0.1"
        );
    }
}
