# bignumber

A Rust library for arbitrary-precision decimal and non-decimal arithmetic

## Install

```sh
cargo add bignumber
```

## Usage

```rs
use bignumber::BigNumber;
use bignumber::errors::BigNumberError;

fn main() -> Result<(), BigNumberError> {
    let a = BigNumber::of("1.0001")?;
    let b = BigNumber::of("4096")?;
    let c = a.pow(&b);
    let d = BigNumber::from(10).powi(18);

    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{} ETH", BigNumber::of("44700000000000000")?.div(&d));

    println!("{}", BigNumber::from(ethereum_types::U256::max_value()));

    Ok(())
}
```
