//! x87 entry values and the x87 state check.
//!
//! A contract may declare up to eight x87 entry values (`st0` first). The
//! worker loads them onto the real FPU stack before calling the original
//! (deepest first, so `st0` ends on top) and copies them to a mirror the
//! rewrite reads through `lf_checker_rt::x87_raw` and its converters. The
//! rewrite side starts with an empty FPU stack: a Rust rewrite cannot pop
//! x87 registers, so it is treated as having consumed every entry value.
//!
//! On return the worker decodes each side's FXSAVE image into an
//! [`X87State`] and [`compare_x87`] requires the same stack top, the same
//! abridged tag byte and bit-identical contents in every valid register.
//! Together with the rewrite's empty start this verifies that the original
//! consumed exactly its x87 arguments and left the same results behind.

/// Most x87 entry values a contract can declare (`st0` to `st7`).
pub const X87_MAX_ENTRIES: usize = 8;
/// Bytes in one 80-bit extended value.
pub const F80_BYTES: usize = 10;
/// Bytes of an FXSAVE image this module reads (through the last x87 slot).
pub const FXSAVE_X87_LEN: usize = FXSAVE_ST0 + 8 * FXSAVE_ST_STRIDE;

/// FXSAVE byte offset of the FPU status word (TOP is bits 11 to 13).
const FXSAVE_FSW: usize = 2;
/// FXSAVE byte offset of the abridged tag byte (bit n set: physical
/// register n holds a value).
const FXSAVE_FTW: usize = 4;
/// FXSAVE byte offset of ST(0); ST(i) follows at a 16-byte stride, in stack
/// order (relative to TOP), not in physical register order.
const FXSAVE_ST0: usize = 32;
/// FXSAVE stride between consecutive ST(i) slots.
const FXSAVE_ST_STRIDE: usize = 16;
/// Shift of the TOP field inside the status word.
const FSW_TOP_SHIFT: u16 = 11;

/// One 80-bit extended value: the 64-bit significand (explicit integer
/// bit at bit 63) and the 16-bit sign and biased exponent.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct F80 {
    /// Significand, integer bit included.
    pub man: u64,
    /// Sign (bit 15) and biased exponent (bits 0 to 14).
    pub sexp: u16,
}

impl F80 {
    /// Build a value from the protocol's three words: significand low and
    /// high halves, then sign and exponent (only the low 16 bits count;
    /// higher bits are rejected so a malformed request is never truncated).
    ///
    /// # Errors
    /// Returns a message when `sexp` does not fit in 16 bits.
    pub fn from_words(lo: u32, hi: u32, sexp: u32) -> Result<F80, String> {
        let sexp = u16::try_from(sexp)
            .map_err(|_| format!("x87 sign/exponent word {sexp:#x} exceeds 16 bits"))?;
        Ok(F80 {
            man: u64::from(lo) | (u64::from(hi) << 32),
            sexp,
        })
    }

    /// The value as the 10 little-endian bytes `fld tbyte` reads.
    #[must_use]
    pub fn to_bytes(self) -> [u8; F80_BYTES] {
        let mut b = [0u8; F80_BYTES];
        b[..8].copy_from_slice(&self.man.to_le_bytes());
        b[8..].copy_from_slice(&self.sexp.to_le_bytes());
        b
    }

    /// Decode 10 little-endian bytes (as `fstp tbyte` writes them).
    #[must_use]
    pub fn from_bytes(b: &[u8; F80_BYTES]) -> F80 {
        let mut m = [0u8; 8];
        m.copy_from_slice(&b[..8]);
        F80 {
            man: u64::from_le_bytes(m),
            sexp: u16::from_le_bytes([b[8], b[9]]),
        }
    }

    /// Lower-case hex of the 10 bytes, for mismatch details.
    #[must_use]
    pub fn hex(self) -> String {
        use std::fmt::Write as _;
        self.to_bytes().iter().fold(String::new(), |mut acc, x| {
            let _ = write!(acc, "{x:02x}");
            acc
        })
    }
}

/// Parse the trial request's x87 entry list (`st0` first, each entry
/// `[lo, hi, sexp]`), enforcing the eight-register limit.
///
/// # Errors
/// Returns a message when more than eight entries are declared, an entry is
/// not three words, or a sign/exponent word does not fit in 16 bits.
pub fn parse_entries(entries: &[Vec<u32>]) -> Result<Vec<F80>, String> {
    if entries.len() > X87_MAX_ENTRIES {
        return Err(format!(
            "x87 declares {} entry values; the FPU stack holds {X87_MAX_ENTRIES}",
            entries.len()
        ));
    }
    entries
        .iter()
        .enumerate()
        .map(|(i, w)| match w.as_slice() {
            [lo, hi, sexp] => F80::from_words(*lo, *hi, *sexp),
            _ => Err(format!(
                "x87 entry st{i} needs three words [lo, hi, sexp], got {}",
                w.len()
            )),
        })
        .collect()
}

/// The x87 state a side left behind, decoded from its FXSAVE image.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct X87State {
    /// Stack top (physical index of ST(0)), 0 to 7.
    pub top: u8,
    /// Abridged tag byte: bit n set when physical register n is valid.
    pub tags: u8,
    /// ST(0) to ST(7) in stack order, valid or not.
    pub regs: [F80; 8],
}

impl X87State {
    /// Decode the x87 part of an FXSAVE image (`None` when it is short).
    #[must_use]
    pub fn from_fxsave(fx: &[u8]) -> Option<X87State> {
        if fx.len() < FXSAVE_X87_LEN {
            return None;
        }
        let fsw = u16::from_le_bytes([fx[FXSAVE_FSW], fx[FXSAVE_FSW + 1]]);
        // Masked to three bits, so the narrowing is exact.
        let top = ((fsw >> FSW_TOP_SHIFT) & 7) as u8;
        let mut regs = [F80::default(); 8];
        for (i, r) in regs.iter_mut().enumerate() {
            let at = FXSAVE_ST0 + i * FXSAVE_ST_STRIDE;
            let mut b = [0u8; F80_BYTES];
            b.copy_from_slice(&fx[at..at + F80_BYTES]);
            *r = F80::from_bytes(&b);
        }
        Some(X87State {
            top,
            tags: fx[FXSAVE_FTW],
            regs,
        })
    }

    /// Whether ST(`i`) holds a value (its physical register is tagged valid).
    #[must_use]
    pub fn valid(&self, i: usize) -> bool {
        let phys = (usize::from(self.top) + i) & 7;
        (self.tags >> phys) & 1 == 1
    }

    /// Number of valid registers.
    #[must_use]
    pub fn depth(&self) -> u32 {
        self.tags.count_ones()
    }
}

/// Compare the final x87 states of the original and the rewrite.
///
/// Passes only when the stack tops, the abridged tag bytes and the 80-bit
/// contents of every valid register are identical. Empty registers are not
/// compared (their bytes are leftovers, not results).
///
/// # Errors
/// Returns the mismatch detail: the tops and tags with the depths, or the
/// first valid register whose bytes differ.
pub fn compare_x87(orig: &X87State, rw: &X87State) -> Result<(), String> {
    if orig.top != rw.top || orig.tags != rw.tags {
        return Err(format!(
            "stack orig top={} tags={:#04x} depth={} rw top={} tags={:#04x} depth={}",
            orig.top,
            orig.tags,
            orig.depth(),
            rw.top,
            rw.tags,
            rw.depth()
        ));
    }
    for i in 0..8 {
        if orig.valid(i) && orig.regs[i] != rw.regs[i] {
            return Err(format!(
                "st{i} orig={} rw={}",
                orig.regs[i].hex(),
                rw.regs[i].hex()
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An FXSAVE image with the given TOP, tag byte and ST(i) contents.
    fn fx(top: u8, tags: u8, regs: &[F80]) -> Vec<u8> {
        let mut v = vec![0u8; 512];
        let fsw = u16::from(top) << FSW_TOP_SHIFT;
        v[FXSAVE_FSW..FXSAVE_FSW + 2].copy_from_slice(&fsw.to_le_bytes());
        v[FXSAVE_FTW] = tags;
        for (i, r) in regs.iter().enumerate() {
            let at = FXSAVE_ST0 + i * FXSAVE_ST_STRIDE;
            v[at..at + F80_BYTES].copy_from_slice(&r.to_bytes());
        }
        v
    }

    const ONE: F80 = F80 {
        man: 0x8000_0000_0000_0000,
        sexp: 0x3FFF,
    };
    const TWO: F80 = F80 {
        man: 0x8000_0000_0000_0000,
        sexp: 0x4000,
    };

    #[test]
    fn words_round_trip_and_reject_wide_exponent() {
        let v = F80::from_words(0x89AB_CDEF, 0x8123_4567, 0xC001).unwrap();
        assert_eq!(v.man, 0x8123_4567_89AB_CDEF);
        assert_eq!(v.sexp, 0xC001);
        assert_eq!(F80::from_bytes(&v.to_bytes()), v);
        assert_eq!(v.hex(), "efcdab896745238101c0");
        assert!(F80::from_words(0, 0, 0x1_0000).is_err());
    }

    #[test]
    fn entries_limit_and_shape() {
        let one = vec![0u32, 0x8000_0000, 0x3FFF];
        assert_eq!(
            parse_entries(std::slice::from_ref(&one)).unwrap(),
            vec![ONE]
        );
        assert!(parse_entries(&vec![one.clone(); 8]).is_ok());
        assert!(parse_entries(&vec![one.clone(); 9]).is_err());
        assert!(parse_entries(&[vec![1, 2]]).is_err());
        assert!(parse_entries(&[]).unwrap().is_empty());
    }

    #[test]
    fn decode_top_tags_and_stack_order() {
        // One value pushed after fninit: TOP 7, physical register 7 valid.
        let s = X87State::from_fxsave(&fx(7, 0x80, &[ONE])).unwrap();
        assert_eq!(s.top, 7);
        assert_eq!(s.depth(), 1);
        assert!(s.valid(0));
        assert!(!s.valid(1));
        assert_eq!(s.regs[0], ONE);
        // Three pushed: TOP 5, physical 5..7 valid; ST(2) is physical 7.
        let s = X87State::from_fxsave(&fx(5, 0xE0, &[ONE, TWO, ONE])).unwrap();
        assert!(s.valid(0) && s.valid(1) && s.valid(2) && !s.valid(3));
        assert!(X87State::from_fxsave(&[0u8; 100]).is_none());
    }

    #[test]
    fn compare_passes_identical_and_ignores_empty_slots() {
        let a = X87State::from_fxsave(&fx(7, 0x80, &[ONE, TWO])).unwrap();
        // ST(1) is empty: its leftover bytes may differ.
        let b = X87State::from_fxsave(&fx(7, 0x80, &[ONE, ONE])).unwrap();
        assert!(compare_x87(&a, &b).is_ok());
        let empty = X87State::from_fxsave(&fx(0, 0, &[])).unwrap();
        assert!(compare_x87(&empty, &empty).is_ok());
    }

    #[test]
    fn compare_catches_depth_top_and_content() {
        let one = X87State::from_fxsave(&fx(7, 0x80, &[ONE])).unwrap();
        let empty = X87State::from_fxsave(&fx(0, 0, &[])).unwrap();
        // Unbalanced stack: one side left a value behind.
        let e = compare_x87(&empty, &one).unwrap_err();
        assert!(e.contains("depth=0") && e.contains("depth=1"), "{e}");
        // Same depth, different physical slot (TOP differs).
        let other = X87State::from_fxsave(&fx(6, 0x40, &[ONE])).unwrap();
        assert!(compare_x87(&one, &other).is_err());
        // Same shape, different bits (sign of a zero counts).
        let pz = F80 { man: 0, sexp: 0 };
        let nz = F80 {
            man: 0,
            sexp: 0x8000,
        };
        let a = X87State::from_fxsave(&fx(7, 0x80, &[pz])).unwrap();
        let b = X87State::from_fxsave(&fx(7, 0x80, &[nz])).unwrap();
        let e = compare_x87(&a, &b).unwrap_err();
        assert!(e.starts_with("st0 "), "{e}");
    }
}
