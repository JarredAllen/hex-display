hex-display
----
[![crates.io](https://img.shields.io/crates/v/hex-display.svg)](https://crates.io/crates/hex-display)

An implementation of Display on a wrapper for `&[u8]` which provides a hexdump (see [`Hex`] and
[`HexDisplayExt`]). This crate also works in `no_std` environments.

If `std` is present, it can also convert to a hexdump as a string.

## Example usage

```rust
use hex_display::HexDisplayExt;

assert_eq!(
    format!("{}", [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef].hex()),
    "0123456789abcdef"
);
#[cfg(feature = "std")]
assert_eq!(
    [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef].hex_string(),
    "0123456789abcdef"
);
```
