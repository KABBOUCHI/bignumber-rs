use core::ops::Mul;

use crate::BigNumber;

impl Mul for BigNumber {
    type Output = BigNumber;

    fn mul(self, rhs: Self) -> Self::Output {
        BigNumber {
            value: self.value.mul(&rhs.value),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::to_bn_safe;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_div() {
        assert_eq!(to_bn_safe!("2").mul(to_bn_safe!("3")).to_string(), "6");
    }
}
