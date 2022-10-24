hex-display
----
[![crates.io](https://img.shields.io/crates/v/hex-display.svg)](https://crates.io/crates/hex-display)

An implementation of Display on a wrapper for `&[u8]` which provides a hexdump (see [`Hex`]).
This crate also works in `no_std` environments.

## Example usage

```rust
use hex_display::Hex;

assert_eq!(
    format!("{}", Hex(&[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef])),
    "0123456789abcdef"
);
```
