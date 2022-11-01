use core::ops::Add;

use crate::BigNumber;

impl Add for BigNumber {
    type Output = BigNumber;

    fn add(self, rhs: Self) -> Self::Output {
        BigNumber {
            value: self.value.add(rhs.value)
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
        assert_eq!(
            to_bn_safe!("1").add(to_bn_safe!("3")).to_string(), 
            "4"
        );
    }

    #[test]
    fn test_b() {
        assert_eq!(
            to_bn_safe!("0.1").add(to_bn_safe!("0.2")).to_string(), 
            "0.3"
        );
    }
}
