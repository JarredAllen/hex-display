#![no_std]

#![doc = include_str!("../README.md")]

use core::fmt::Display;

/// A wrapper type for `&[u8]` which implements Display by providing a hexdump
///
/// ```
/// use hex_display::Hex;
///
/// assert_eq!(
///     format!("{}", Hex(&[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef])),
///     "0123456789abcdef"
/// );
/// ```
#[derive(Debug)]
pub struct Hex<'a>(
    /// The bytes to be converted into a hexdump
    pub &'a [u8]
);

impl<'a> Display for Hex<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for byte in self.0 {
            f.write_fmt(format_args!("{:02x}", byte))?;
        }
        Ok(())
    }
}

/// Like [`Hex`], but provides an upper-case hexdump
///
/// ```
/// use hex_display::UpperHex;
///
/// assert_eq!(
///     format!("{}", UpperHex(&[0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF])),
///     "0123456789ABCDEF"
/// );
/// ```
#[derive(Debug)]
pub struct UpperHex<'a>(
    /// The bytes to be converted into a hexdump
    pub &'a [u8]
);

impl<'a> Display for UpperHex<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for byte in self.0 {
            f.write_fmt(format_args!("{:02X}", byte))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use std::format;

    use super::*;

    #[test]
    fn test_all_bytes() {
        for byte in 0..=0xff {
            assert_eq!(format!("{:02x}", byte), format!("{}", Hex(&[byte])));
            assert_eq!(format!("{:02X}", byte), format!("{}", UpperHex(&[byte])));
        }
    }

    #[test]
    fn test_all_byte_pairs() {
        for (a, b) in (0..=0xff).zip(0..=0xff) {
            assert_eq!(format!("{:02x}{:02x}", a, b), format!("{}", Hex(&[a, b])));
            assert_eq!(format!("{:02X}{:02X}", a, b), format!("{}", UpperHex(&[a, b])));
        }
    }
}
