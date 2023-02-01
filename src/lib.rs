#![no_std]
#![doc = include_str!("../README.md")]

use core::fmt::{Debug, Display, LowerHex, UpperHex};

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
///     format!("{:X}", [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef].hex()),
///     "0123456789ABCDEF"
/// );
/// ```
pub trait HexDisplayExt {
    /// Display as a hexdump
    fn hex(&self) -> Hex<'_>;

    /// Convert to a upper-case hex string
    ///
    /// Only present when built with `std` support.
    #[cfg(feature = "std")]
    fn upper_hex_string(&self) -> std::string::String {
        use std::string::ToString;
        format!("{:X}", self.hex())
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
/// By default, it outputs a lower-case hexdump, but it outputs upper-case if provided with `{:X}`
/// formatting option.
///
/// ```
/// use hex_display::Hex;
///
/// assert_eq!(
///     format!("{}", Hex(&[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef])),
///     "0123456789abcdef"
/// );
/// assert_eq!(
///     format!("{:?}", Hex(&[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef])),
///     "0123456789abcdef"
/// );
/// assert_eq!(
///     format!("{:X}", Hex(&[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef])),
///     "0123456789ABCDEF"
/// );
/// ```
pub struct Hex<'a>(
    /// The bytes to be converted into a hexdump
    pub &'a [u8],
);

impl<'a> UpperHex for Hex<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for byte in self.0 {
            f.write_fmt(format_args!("{:02X}", byte))?;
        }
        Ok(())
    }
}
impl<'a> LowerHex for Hex<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for byte in self.0 {
            f.write_fmt(format_args!("{:02x}", byte))?;
        }
        Ok(())
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

#[cfg(test)]
mod tests {
    extern crate std;

    use std::format;

    use super::*;

    #[test]
    fn test_all_bytes() {
        for byte in 0..=0xff {
            assert_eq!(format!("{:02x}", byte), format!("{}", Hex(&[byte])));
            assert_eq!(format!("{:02X}", byte), format!("{:X}", Hex(&[byte])));
        }
    }

    #[test]
    fn test_all_byte_pairs() {
        for (a, b) in (0..=0xff).zip(0..=0xff) {
            assert_eq!(format!("{:02x}{:02x}", a, b), format!("{}", Hex(&[a, b])));
            assert_eq!(format!("{:02X}{:02X}", a, b), format!("{:X}", Hex(&[a, b])));
        }
    }
}
