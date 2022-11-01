use crate::BigNumber;

impl From<usize> for BigNumber {
    #[inline]
    fn from(n: usize) -> Self {
        BigNumber::of(n.to_string().as_str()).unwrap()
    }
}
