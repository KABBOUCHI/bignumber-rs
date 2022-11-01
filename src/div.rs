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
            "0.33333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333328864064023352467633717604575363253587689985995765203718876693305655184676735981074126254996648831498545386397509092504902336212931439869329033650874176853"
        );
    }
}
