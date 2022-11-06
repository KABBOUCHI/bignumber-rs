use core::ops::Div;

use crate::BigNumber;

impl Div for BigNumber {
    type Output = BigNumber;

    fn div(self, rhs: BigNumber) -> Self::Output {
        BigNumber {
            value: self.value.div(rhs.value),
        }
    }
}

impl Div<&BigNumber> for BigNumber {
    type Output = BigNumber;

    fn div(self, rhs: &BigNumber) -> Self::Output {
        BigNumber {
            value: self.value.div(rhs.value.clone()),
        }
    }
}

impl Div for &BigNumber {
    type Output = BigNumber;

    fn div(self, rhs: &BigNumber) -> Self::Output {
        BigNumber {
            value: self.value.clone().div(rhs.value.clone()),
        }
    }
}

impl Div<BigNumber> for &BigNumber {
    type Output = BigNumber;

    fn div(self, rhs: BigNumber) -> Self::Output {
        BigNumber {
            value: self.value.clone().div(rhs.value),
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
        assert_eq!(
            to_bn_safe!("1").div(to_bn_safe!("3")).to_string(), 
            "0.3333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333294735970254227934860902038908899590970188601585855700792214923812183634957653"
        );
    }
}
