use std::fmt::Display;

use crate::BigNumber;

impl Display for BigNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}
