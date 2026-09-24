# Changelog

## 0.1.0

Full rewrite of the internals; **breaking** relative to `0.0.x`.

### Fixed
- Fixed a stack-overflow bug: `PartialOrd::le`/`::ge` were defined as
  `self <= rhs` / `self >= rhs`, which is infinite recursion. `cargo test`
  previously crashed on `test_ordering`. `Ord`/`PartialOrd` are now derived
  directly from the single `raw_value` field.
- Fixed silent integer overflow: arithmetic operators previously wrapped
  silently in release builds while panicking in debug builds. They now use
  `checked_*` internally and panic consistently in both.
- Fixed precision loss in division: `Div`/`DivAssign` previously round-tripped
  through `f32`, which loses precision on values as small as a few million
  cents. Division is now done with exact integer arithmetic and correct
  round-half-away-from-zero rounding.
- Fixed a formatting bug in `add_commas` where negative numbers produced a
  misplaced comma next to the sign (e.g. `-100000` rendered as
  `-,100,000`).

### Changed
- Internal representation widened from `i32` to `i64` cents (~±$21.5M limit
  before, ~±$92 quadrillion now).
- `Greenback::new()` now returns `Result<Greenback, GreenbackError>` instead
  of panicking on out-of-range cents.
- `Greenback::from_float()` removed; use the new `FromStr` implementation
  (`"12.34".parse::<Greenback>()`) instead, which avoids floating-point
  representation error entirely.
- Crate now targets the 2024 edition (MSRV 1.85), replacing the implicit
  2015 edition and its `extern crate` / bare `use` idioms.
- CI moved from Travis CI (defunct) to GitHub Actions, running
  `cargo fmt --check`, `cargo clippy -- -D warnings`, and
  `cargo test`/`cargo test --release` on stable/beta/nightly.

### Added
- `checked_add`, `checked_sub`, `checked_mul`, `checked_div` for callers who
  want `Option`-based overflow/divide-by-zero handling instead of panics.
- `Neg`, `Hash`, and `Default` implementations.
- `GreenbackError` (implements `std::error::Error`) covering invalid cents
  and parse failures.
- Regression tests for the overflow, precision, and comma-formatting bugs
  above, plus tests for the new `FromStr`, `Neg`, `Default`, and checked-*
  APIs.

## 0.0.3 and earlier

See git history.
