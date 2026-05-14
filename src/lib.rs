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
///
/// The formatter's width/alignment/fill are honored, so the output can be
/// padded like any other [`Display`] value:
///
/// ```
/// use hex_display::HexDisplayExt;
/// assert_eq!(format!("{:>8}", [0x01, 0x23].hex()), "    0123");
/// assert_eq!(format!("{:*^8}", [0x01, 0x23].hex()), "**0123**");
/// ```
///
/// Use the alternate form (`#`) to emit a multiline `hexdump -C`-style dump
/// with an offset column and ASCII gutter:
///
/// ```
/// use hex_display::HexDisplayExt;
/// assert_eq!(
///     format!("{:#}", b"Hello, world!\n".hex()),
///     "00000000  48 65 6c 6c 6f 2c 20 77  6f 72 6c 64 21 0a        |Hello, world!.|"
/// );
/// ```
///
/// See the documentation for [`Hex`] for more details.
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

    /// Convert to a upper-case hexdump.
    ///
    /// Only present when built with `alloc` support.
    #[cfg(feature = "alloc")]
    fn upper_hexdump(&self) -> alloc::string::String {
        alloc::format!("{:#X}", self.hex())
    }

    /// Convert to a lower-case hexdump.
    ///
    /// Only present when built with `alloc` support.
    #[cfg(feature = "alloc")]
    fn hexdump(&self) -> alloc::string::String {
        alloc::format!("{:#x}", self.hex())
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
///
/// The formatter's `width`, `align`, and `fill` are applied to the entire
/// output, the same way they would be for any other [`Display`] value:
///
/// ```
/// use hex_display::Hex;
/// assert_eq!(format!("{:>8}", Hex(&[0x01, 0x23])), "    0123");
/// ```
///
/// Passing the alternate form (`#`) switches the output to a multiline
/// `hexdump -C`-style dump, with an 8-digit offset column, two groups of
/// eight hex bytes, and an ASCII gutter (non-printable bytes shown as `.`):
///
/// ```
/// use hex_display::Hex;
///
/// let bytes: [u8; 18] = [
///     0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
///     0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
///     0x10, 0x11,
/// ];
/// assert_eq!(
///     format!("{:#}", Hex(&bytes)),
///     "\
/// 00000000  00 01 02 03 04 05 06 07  08 09 0a 0b 0c 0d 0e 0f  |................|
/// 00000010  10 11                                             |..|",
/// );
/// ```
///
/// The alternate form combines with the upper-case flag (`{:#X}`) and with
/// width-padding (`{:>WIDTH$#}`) just like the single-line form.
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

/// Bytes per line in the multiline hexdump output.
const HEXDUMP_LINE_BYTES: usize = 16;
/// Width of the hex region in a hexdump line: two 8-byte groups (each
/// `"xx "` × 8 = 24 chars) separated by an extra space.
const HEXDUMP_HEX_REGION: usize = 8 * 3 * 2;
/// Width of a full hexdump line (16 bytes), excluding any trailing newlines.
///
/// Layout: `ADDRESS_␠␠<hex region>␠␠|<ascii 16>|`.
const HEXDUMP_FULL_LINE: usize = 8 + 2 + HEXDUMP_HEX_REGION + 2 + 1 + HEXDUMP_LINE_BYTES + 1;

/// Compute the exact length, in bytes, of the alternate-form hexdump output
/// for a slice of `byte_count` bytes (no trailing newline).
fn hexdump_len(byte_count: usize) -> usize {
    if byte_count == 0 {
        return 0;
    }
    let n_full = byte_count / HEXDUMP_LINE_BYTES;
    let rem = byte_count % HEXDUMP_LINE_BYTES;
    let n_lines = n_full + usize::from(rem > 0);
    // Partial line: same offset + hex region as a full line, but the ASCII
    // gutter only contains `rem` bytes (still bracketed by `|`).
    let partial = if rem > 0 {
        8 + 2 + HEXDUMP_HEX_REGION + 2 + 1 + rem + 1
    } else {
        0
    };
    n_full * HEXDUMP_FULL_LINE + partial + n_lines - 1
}

/// Write the multiline `hexdump -C`-style output for `bytes` into `f`.
fn write_hexdump(
    f: &mut core::fmt::Formatter<'_>,
    bytes: &[u8],
    table: &[u8; 16],
) -> core::fmt::Result {
    for (line_idx, chunk) in bytes.chunks(HEXDUMP_LINE_BYTES).enumerate() {
        // Write out in line-based chunks, to save overhead.
        //
        // Initialize to all spaces so we can just skip slots and they'll be a space.
        let mut buf = [b' '; HEXDUMP_FULL_LINE + 1];
        let mut buf_idx = 0;
        if line_idx > 0 {
            buf[buf_idx] = b'\n';
            buf_idx += 1;
        }

        let offset = line_idx * HEXDUMP_LINE_BYTES;
        let offset_buf = &mut buf[buf_idx..][..8];
        for i in 0..8 {
            offset_buf[7 - i] = table[(offset >> (i * 4)) & 0xf];
        }
        buf_idx += 8 + 2; // add 2 blank spaces after above

        let hex_buf = &mut buf[buf_idx..][..HEXDUMP_HEX_REGION];
        for (i, &byte) in chunk.iter().enumerate() {
            // Bytes 8..16 sit one extra space to the right to form two groups.
            let pos = i * 3 + usize::from(i >= 8);
            hex_buf[pos] = table[(byte >> 4) as usize];
            hex_buf[pos + 1] = table[(byte & 0x0f) as usize];
        }
        buf_idx += HEXDUMP_HEX_REGION + 2;
        buf[buf_idx] = b'|';
        buf_idx += 1;

        let ascii_buf = &mut buf[buf_idx..][..chunk.len()];
        for (i, &byte) in chunk.iter().enumerate() {
            ascii_buf[i] = if (0x20..=0x7e).contains(&byte) {
                byte
            } else {
                b'.'
            };
        }
        buf_idx += chunk.len();
        buf[buf_idx] = b'|';
        buf_idx += 1;
        f.write_str(
            core::str::from_utf8(&buf[..buf_idx]).expect("we should not write invalid ascii"),
        )?;
    }
    Ok(())
}

/// Apply the formatter's width/alignment/fill around `write_content`.
///
/// Acts like [`core::fmt::Formatter::pad`] but without requiring the content
/// to first be materialized into a `&str` — the caller provides the exact
/// `content_len` and a closure that writes the content directly.
fn write_padded(
    f: &mut core::fmt::Formatter<'_>,
    content_len: usize,
    write_content: impl FnOnce(&mut core::fmt::Formatter<'_>) -> core::fmt::Result,
) -> core::fmt::Result {
    use core::fmt::{Alignment, Write};
    let Some(width) = f.width() else {
        return write_content(f);
    };
    if content_len >= width {
        return write_content(f);
    }
    let pad_total = width - content_len;
    let fill = f.fill();
    let align = f.align().unwrap_or(Alignment::Left);
    let (pre, post) = match align {
        Alignment::Left => (0, pad_total),
        Alignment::Right => (pad_total, 0),
        Alignment::Center => (pad_total / 2, pad_total - pad_total / 2),
    };
    for _ in 0..pre {
        f.write_char(fill)?;
    }
    write_content(f)?;
    for _ in 0..post {
        f.write_char(fill)?;
    }
    Ok(())
}

/// Dispatch to either the single-line hex encoding or the alternate-form
/// multiline hexdump, then apply any width-padding requested by the formatter.
fn fmt_hex_with_options(
    f: &mut core::fmt::Formatter<'_>,
    bytes: &[u8],
    table: &[u8; 16],
) -> core::fmt::Result {
    let alternate = f.alternate();
    let content_len = if alternate {
        hexdump_len(bytes.len())
    } else {
        bytes.len() * 2
    };
    write_padded(f, content_len, |f| {
        if alternate {
            write_hexdump(f, bytes, table)
        } else {
            write_hex(f, bytes, table)
        }
    })
}

impl UpperHex for Hex<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fmt_hex_with_options(f, self.0, UPPER_HEX)
    }
}
impl LowerHex for Hex<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fmt_hex_with_options(f, self.0, LOWER_HEX)
    }
}
impl Debug for Hex<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fmt_hex_with_options(f, self.0, LOWER_HEX)
    }
}
impl Display for Hex<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fmt_hex_with_options(f, self.0, LOWER_HEX)
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
            #[cfg(feature = "alloc")]
            assert_eq!(format!("{:02x}", byte), [byte].hex_string());
            assert_eq!(format!("{:02x}", byte), format!("{:x}", Hex(&[byte])));
            assert_eq!(format!("{:02X}", byte), format!("{:X}", Hex(&[byte])));
            #[cfg(feature = "alloc")]
            assert_eq!(format!("{:02X}", byte), [byte].upper_hex_string());
        }
    }

    #[test]
    fn test_all_byte_pairs() {
        for (a, b) in (0..=0xff).zip(0..=0xff) {
            assert_eq!(format!("{:02x}{:02x}", a, b), format!("{}", Hex(&[a, b])));
            assert_eq!(format!("{:02X}{:02X}", a, b), format!("{:X}", Hex(&[a, b])));
        }
    }

    #[test]
    fn test_width_padding() {
        let h = Hex(&[0x01, 0x23]);
        assert_eq!(format!("{:>8}", h), "    0123");
        assert_eq!(format!("{:<8}", h), "0123    ");
        assert_eq!(format!("{:^8}", h), "  0123  ");
        assert_eq!(format!("{:*>8}", h), "****0123");
        // Width <= content length is a no-op.
        assert_eq!(format!("{:2}", h), "0123");
        // Default alignment is left, matching `Formatter::pad` for strings.
        assert_eq!(format!("{:8}", h), "0123    ");
        // Padding applies to UpperHex as well.
        assert_eq!(format!("{:>6X}", h), "  0123");
    }

    #[test]
    fn test_alternate_single_line() {
        let bytes: [u8; 4] = [b'a', b'b', 0x00, 0xff];
        let expected = "00000000  61 62 00 ff                                       |ab..|";
        assert_eq!(format!("{:#}", Hex(&bytes)), expected);
        assert_eq!(format!("{:#?}", Hex(&bytes)), expected);
    }

    #[test]
    fn test_alternate_multiline() {
        let mut bytes = [0u8; 18];
        for (i, b) in bytes.iter_mut().enumerate() {
            *b = i as u8;
        }
        let expected = "\
00000000  00 01 02 03 04 05 06 07  08 09 0a 0b 0c 0d 0e 0f  |................|
00000010  10 11                                             |..|";
        assert_eq!(format!("{:#}", Hex(&bytes)), expected);
        #[cfg(feature = "alloc")]
        assert_eq!(bytes.hexdump(), expected);
    }

    #[test]
    fn test_alternate_upper_and_ascii_gutter() {
        let bytes = b"Hello!\x00\x7f";
        let expected = "00000000  48 65 6C 6C 6F 21 00 7F                           |Hello!..|";
        assert_eq!(format!("{:#X}", bytes.hex()), expected);
        #[cfg(feature = "alloc")]
        assert_eq!(bytes.upper_hexdump(), expected);
    }

    #[test]
    fn test_alternate_empty() {
        assert_eq!(format!("{:#}", Hex(&[])), "");
    }

    #[test]
    fn test_alternate_with_padding() {
        let bytes = [0xabu8, 0xcd];
        let dump = format!("{:#}", Hex(&bytes));
        assert_eq!(dump.len(), hexdump_len(bytes.len()));
        // Right-align the whole multiline hexdump in a wider field.
        let target_width = dump.len() + 4;
        let padded = format!("{:>#1$}", Hex(&bytes), target_width);
        assert_eq!(padded.len(), target_width);
        assert!(padded.starts_with("    "));
        assert_eq!(&padded[4..], dump);
    }
}
