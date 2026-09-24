[![greenback on crates.io](https://img.shields.io/crates/v/greenback.svg)](https://crates.io/crates/greenback)

# greenback

Library for safely handling USD values as integers.

## Installation

Edit `Cargo.toml`:

```toml
[dependencies]
greenback = "0.1"
```

## Usage

```rust
use greenback::Greenback;

fn main() {

    // regular arithmetic operations

    let unit_price = Greenback::new(10, 99).unwrap(); // $10.99
    let quantity = 10;
    let coupon = Greenback::new(2, 0).unwrap(); // $2.00 off

    let total_cost = unit_price * quantity - coupon;

    println!("Total cost: {}", total_cost);

    // summation example

    let foo: Greenback = "1.23".parse().unwrap(); // $1.23
    let bar = Greenback::from_cents(4_56); // $4.56
    let baz = Greenback::new(3, 50).unwrap(); // $3.50

    let sum: Greenback = vec![foo, bar, baz].into_iter().sum();

    if sum > Greenback::zero() {
        println!("sum: {}", sum);
    }
}
```

Output:
```
Total cost: $107.90
sum: $9.29
```

## Status

All the basic arithmetic operations (`Add`, `Sub`, `Mul`, `Div`, and their
`*Assign` variants) are implemented, along with `Ord`, `Eq`, `Hash`,
`Default`, `Neg`, `Sum`, and `FromStr`. Values are stored internally as an
`i64` count of cents (~±$92 quadrillion range) and all arithmetic is
overflow-checked: operators panic consistently on overflow or division by
zero (in both debug and release builds), and `checked_add`/`checked_sub`/
`checked_mul`/`checked_div` are available for callers who want to handle
that case without panicking.

There's a default formatter which prints values like `$1,234.56`, and a
`FromStr` implementation that parses strings like `"12.34"`, `"$1,234.56"`,
or `"-$1,234.56"`.

Pull requests and issues are very welcome.

### 0.1.0 breaking changes

`0.1.0` rewrites most of the internals and is **not** backwards compatible
with `0.0.x`:

- The internal representation moved from `i32` to `i64` cents, and all
  public integer parameters/returns (`dollars()`, `cents()`, `raw_value()`,
  `Mul`/`Div` right-hand sides) changed from `i32` to `i64` accordingly.
- `Greenback::new()` now returns `Result<Greenback, GreenbackError>` instead
  of panicking on invalid cents.
- `Greenback::from_float()` was removed in favor of a `FromStr`
  implementation, since converting through a float reintroduced the
  precision problems this crate exists to avoid.
- Division no longer round-trips through `f32` internally, so results on
  large values are now exact instead of occasionally off by one cent.
- Arithmetic operators now panic consistently on overflow in *both* debug
  and release builds (previously release builds silently wrapped).
- The crate now targets Rust 2024 edition; MSRV is 1.85.

See `CHANGELOG.md` for the full list.
