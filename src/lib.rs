#![no_std]
#![doc = include_str!("../README.md")]

use core::fmt::{Debug, Display, LowerHex, UpperHex};

#[cfg(any(feature = "std", test))]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

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
    /// Only present when built with `alloc` support.
    #[cfg(feature = "alloc")]
    fn upper_hex_string(&self) -> alloc::string::String {
        alloc::format!("{:X}", self.hex())
    }

    /// Convert to a lower-case hex string
    ///
    /// Only present when built with `alloc` support.
    #[cfg(feature = "alloc")]
    fn hex_string(&self) -> alloc::string::String {
        use alloc::string::ToString;
        self.hex().to_string()
    }
}

impl HexDisplayExt for [u8] {
    fn hex(&self) -> Hex<'_> {
        Hex(self)
    }
}

impl<T: AsRef<[u8]>> HexDisplayExt for T {
    fn hex(&self) -> Hex<'_> {
        Hex(self.as_ref())
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

/// Lookup table indexed by nibble for lowercase hex output.
const LOWER_HEX: &[u8; 16] = b"0123456789abcdef";
/// Uppercase counterpart to [`LOWER_HEX`].
const UPPER_HEX: &[u8; 16] = b"0123456789ABCDEF";

/// Format `bytes` as hex into `f` using the supplied nibble lookup table.
///
/// Bytes are encoded into a stack buffer in chunks so we make one
/// [`core::fmt::Write::write_str`] call per chunk instead of one formatting
/// call per byte. The chunk size is chosen to comfortably fit on the stack
/// while amortizing the formatter's per-call overhead.
fn write_hex(
    f: &mut core::fmt::Formatter<'_>,
    bytes: &[u8],
    table: &[u8; 16],
) -> core::fmt::Result {
    /// Bytes per chunk; produces `CHUNK * 2` ASCII hex characters per write.
    const CHUNK: usize = 32;
    let mut buf = [0u8; CHUNK * 2];
    for chunk in bytes.chunks(CHUNK) {
        for (i, &byte) in chunk.iter().enumerate() {
            buf[i * 2] = table[(byte >> 4) as usize];
            buf[i * 2 + 1] = table[(byte & 0x0f) as usize];
        }
        // The lookup table should be valid ascii, so this shouldn't panic, but is kept to
        // safeguard against future bugs.
        //
        // Switching to `unsafe` was profiled and produces ~20% speedup, which was deemed to be not
        // enough speedup to be worth introducing new `unsafe`.
        let s = core::str::from_utf8(&buf[..chunk.len() * 2])
            .expect("hex lookup table only emits ASCII");
        f.write_str(s)?;
    }
    Ok(())
}

impl UpperHex for Hex<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write_hex(f, self.0, UPPER_HEX)
    }
}
impl LowerHex for Hex<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write_hex(f, self.0, LOWER_HEX)
    }
}
impl Debug for Hex<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write_hex(f, self.0, LOWER_HEX)
    }
}
impl Display for Hex<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write_hex(f, self.0, LOWER_HEX)
    }
}

#[cfg(test)]
mod tests {
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
