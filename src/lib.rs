#![no_std]
#![doc = include_str!("../README.md")]

use core::fmt::{Display, Debug};

#[cfg(feature = "std")]
extern crate std;

/// An extension trait that allows for more easily constructing [`Hex`] values
///
/// ```
/// use hex_display::HexDisplayExt;
/// assert_eq!(
///     format!("{}", [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef].hex()),
///     "0123456789abcdef"
/// );
/// assert_eq!(
///     format!("{}", [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef].upper_hex()),
///     "0123456789ABCDEF"
/// );
/// ```
pub trait HexDisplayExt {
    /// Display as a hexdump
    fn hex(&self) -> Hex<'_>;

    /// Display as a capitalized hexdump
    fn upper_hex(&self) -> UpperHex<'_> {
        self.hex().upper()
    }

    /// Convert to a lower-case hex string
    ///
    /// Only present when built with `std` support.
    #[cfg(feature = "std")]
    fn hex_string(&self) -> std::string::String {
        use std::string::ToString;
        self.hex().to_string()
    }
}

impl HexDisplayExt for [u8] {
    fn hex(&self) -> Hex<'_> {
        Hex(self)
    }
}

impl<const N: usize> HexDisplayExt for [u8; N] {
    fn hex(&self) -> Hex<'_> {
        Hex(self)
    }
}

/// A wrapper type for `&[u8]` which implements Display by providing a hexdump
///
/// See [`HexDisplayExt`] for an easier method of constructing this type.
///
/// ```
/// use hex_display::Hex;
///
/// assert_eq!(
///     format!("{}", Hex(&[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef])),
///     "0123456789abcdef"
/// );
/// ```
pub struct Hex<'a>(
    /// The bytes to be converted into a hexdump
    pub &'a [u8],
);

impl<'a> Hex<'a> {
    /// Convert to upper-case hexdump
    pub fn upper(self) -> UpperHex<'a> {
        UpperHex(self.0)
    }

    /// Convert to lower-case hexdump
    ///
    /// This is a no-op.
    pub fn lower(self) -> Self {
        self
    }
}

impl<'a> Debug for Hex<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for byte in self.0 {
            f.write_fmt(format_args!("{:02x}", byte))?;
        }
        Ok(())
    }
}
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
pub struct UpperHex<'a>(
    /// The bytes to be converted into a hexdump
    pub &'a [u8],
);

impl<'a> UpperHex<'a> {
    /// Convert to upper-case hexdump
    ///
    /// This is a no-op.
    pub fn upper(self) -> Self {
        self
    }

    /// Convert to lower-case hexdump
    pub fn lower(self) -> Hex<'a> {
        Hex(self.0)
    }
}

impl<'a> Debug for UpperHex<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for byte in self.0 {
            f.write_fmt(format_args!("{:02X}", byte))?;
        }
        Ok(())
    }
}
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
            assert_eq!(
                format!("{:02X}{:02X}", a, b),
                format!("{}", UpperHex(&[a, b]))
            );
        }
    }
}
