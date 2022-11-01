# bignumber (WIP)

A Rust library for arbitrary-precision decimal and non-decimal arithmetic built on top of [dashu-float](https://github.com/cmpute/dashu/tree/master/float)

## Install

```sh
cargo add bignumber
```

## Usage

```rs
use std::ops::Div;

use bignumber::{BigNumber, BigNumberError};

fn main() -> Result<(), BigNumberError> {
    let a = BigNumber::of("1.0001")?;
    let b = BigNumber::of("4096")?;
    let c = a.pow(&b);
    let d = BigNumber::from(10).powi(18);
    let e = ethereum_types::U256::max_value();
    let f = BigNumber::from(e);

    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{} ETH", BigNumber::of("44700000000000000")?.div(d));
    println!("{}", f);

    Ok(())
}
```
