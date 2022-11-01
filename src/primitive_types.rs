use crate::BigNumber;
use primitive_types::{U128, U256, U512};

impl From<U128> for BigNumber {
    #[inline]
    fn from(n: U128) -> Self {
        BigNumber::of(n.to_string().as_str()).unwrap()
    }
}

impl From<U256> for BigNumber {
    #[inline]
    fn from(n: U256) -> Self {
        BigNumber::of(n.to_string().as_str()).unwrap()
    }
}

impl From<U512> for BigNumber {
    #[inline]
    fn from(n: U512) -> Self {
        BigNumber::of(n.to_string().as_str()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::BigNumber;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_a() {
        let a = U256::from_dec_str("42").unwrap();
        assert_eq!(BigNumber::from(a).to_string(), "42");
    }

    #[test]
    fn test_b() {
        let b = U256::from(0x42);

        assert_eq!(BigNumber::from(b).to_string(), "66");
    }

    #[test]
    fn test_c() {
        let c = U256::from("0x69");

        assert_eq!(BigNumber::from(c).to_string(), "105");
    }

    #[test]
    fn test_d() {
        assert_eq!(
            BigNumber::from( U512::max_value()).to_string(),
            "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095"
        );
    }
}
