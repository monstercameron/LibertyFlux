//! `lf-lift-diff`: differential tests of lifted functions against their
//! verified 32-bit rewrites.
//!
//! The method is specified in the `lf-lift` crate documentation (section
//! *Proof*). This crate holds the machinery:
//!
//! - [`rewrites`] compiles the verified rewrite files from
//!   `rewrites/verified/functions/` unchanged, with `include!`.
//! - [`rt`] is the runtime those files call. On the 32-bit target it is the
//!   real `lf-checker-rt`: the test points its relocated base at a test
//!   buffer ([`VaImage`]) and its callee table at recording stubs. On any
//!   other host a stand-in with the same surface runs the same files: its
//!   `global()` points into the test buffer, its `relocated()` answers with
//!   a 32-bit number that depends on the buffer as the real answer does,
//!   and its callee macros record directly. Rewrites that turn a relocated
//!   address into a pointer themselves need real 32-bit addresses and run
//!   only on the 32-bit target.
//! - Addresses from runs against different buffers are in different
//!   address spaces. A case that compares address-valued results or callee
//!   arguments runs both sides against the same [`VaImage`], reset to the
//!   same starting bytes before each run.
//! - [`check`] runs a reference, a lift and a deliberately wrong lift over
//!   generated inputs ([`Rng`]) and asserts that the first two agree on
//!   every input and the wrong one is caught on at least one.
//! - [`F32`] and [`F64`] compare floats bit for bit, with any NaN equal to
//!   any NaN (the lifted contract leaves NaN payloads to the hardware).
//!
//! The differential cases themselves are in `tests/diff.rs`.
//!
//! Build and run on the 32-bit target (a Windows runner):
//! `cargo test --target i686-pc-windows-msvc -p lf-lift-diff`. On any
//! other host, `cargo test -p lf-lift-diff` runs every case except those
//! marked 32-bit only.

use core::fmt;

use lf_core::boundary::{BoundaryError, Image32};

pub mod rewrites;
pub mod rt;

pub use rt::Call;

/// A small deterministic generator (`SplitMix64`) for test inputs.
#[derive(Clone, Debug)]
pub struct Rng(u64);

/// Words every integer generator mixes in: zero, small values, sign and
/// width boundaries, all ones.
pub const EDGE_U32: [u32; 16] = [
    0,
    1,
    2,
    3,
    0x7F,
    0x80,
    0xFF,
    0x100,
    0x7FFF,
    0x8000,
    0xFFFF,
    0x1_0000,
    0x7FFF_FFFF,
    0x8000_0000,
    0xFFFF_FFFE,
    u32::MAX,
];

/// Float bit patterns every float generator mixes in: signed zeros, one,
/// halves, the largest and smallest normals, a subnormal, infinities, a
/// quiet and a signalling NaN with payloads, and a negative NaN.
pub const EDGE_F32: [u32; 16] = [
    0x0000_0000,
    0x8000_0000,
    0x3F80_0000,
    0xBF80_0000,
    0x3F00_0000,
    0xBF00_0000,
    0x3FC0_0000,
    0x4040_0000,
    0x7F7F_FFFF,
    0x0080_0000,
    0x0000_0001,
    0x7F80_0000,
    0xFF80_0000,
    0x7FC0_1234,
    0x7FA0_0001,
    0xFFC0_0000,
];

impl Rng {
    /// A generator seeded with `seed`.
    #[must_use]
    pub const fn new(seed: u64) -> Self {
        Self(seed)
    }

    /// Next 64 random bits.
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Next 32 random bits.
    pub fn u32(&mut self) -> u32 {
        // The high half of a SplitMix64 output is as good as the low.
        (self.next_u64() >> 32) as u32
    }

    /// A value below `n` (`n` must be non-zero).
    pub fn below(&mut self, n: u32) -> u32 {
        self.u32() % n
    }

    /// True with probability `1 / n`.
    pub fn one_in(&mut self, n: u32) -> bool {
        self.below(n) == 0
    }

    /// An element of `items`.
    ///
    /// # Panics
    ///
    /// When `items` is empty or longer than `u32::MAX`.
    pub fn pick<T: Copy>(&mut self, items: &[T]) -> T {
        let n = u32::try_from(items.len()).expect("pick from a short slice");
        items[self.below(n) as usize]
    }

    /// A word that is an edge value a quarter of the time, a small value a
    /// quarter of the time, and random otherwise.
    pub fn edge_u32(&mut self) -> u32 {
        match self.below(4) {
            0 => self.pick(&EDGE_U32),
            1 => self.below(0x200),
            _ => self.u32(),
        }
    }

    /// A float that is an edge pattern a quarter of the time, a small
    /// value with a fraction a quarter of the time, and random bits
    /// otherwise.
    #[allow(clippy::cast_precision_loss)] // small integers convert exactly
    pub fn edge_f32(&mut self) -> f32 {
        match self.below(4) {
            0 => f32::from_bits(self.pick(&EDGE_F32)),
            1 => {
                let whole = self.below(64) as f32 - 32.0;
                whole + self.pick(&[0.0, 0.25, 0.5, 0.75, 0.999])
            }
            _ => f32::from_bits(self.u32()),
        }
    }
}

/// `n` inputs from `generate`, seeded with `seed`.
pub fn inputs<I>(seed: u64, n: usize, mut generate: impl FnMut(&mut Rng) -> I) -> Vec<I> {
    let mut rng = Rng::new(seed);
    (0..n).map(|_| generate(&mut rng)).collect()
}

/// An `f32` compared bit for bit, with every NaN equal to every NaN.
#[derive(Clone, Copy)]
pub struct F32(pub f32);

impl PartialEq for F32 {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits() || (self.0.is_nan() && other.0.is_nan())
    }
}

impl fmt::Debug for F32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ({:#010x})", self.0, self.0.to_bits())
    }
}

/// An `f64` compared like [`F32`].
#[derive(Clone, Copy)]
pub struct F64(pub f64);

impl PartialEq for F64 {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits() || (self.0.is_nan() && other.0.is_nan())
    }
}

impl fmt::Debug for F64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ({:#018x})", self.0, self.0.to_bits())
    }
}

/// Runs one differential case.
///
/// For every input, `reference` (the verified rewrite, through whatever
/// translation of its result the case needs) and `lifted` must agree. Then
/// `wrong`, a deliberately wrong lift, must disagree with `reference` on at
/// least one input: a case whose wrong version passes cannot see the
/// difference it was built to show.
///
/// # Panics
///
/// On the first disagreement, naming the case and the input, or when the
/// wrong version is not caught.
pub fn check<I: fmt::Debug, O: PartialEq + fmt::Debug>(
    name: &str,
    inputs: &[I],
    mut reference: impl FnMut(&I) -> O,
    mut lifted: impl FnMut(&I) -> O,
    mut wrong: impl FnMut(&I) -> O,
) {
    assert!(!inputs.is_empty(), "{name}: no inputs");
    for input in inputs {
        let expected = reference(input);
        let got = lifted(input);
        assert_eq!(
            got, expected,
            "{name}: lifted form differs on input {input:?}"
        );
    }
    let caught = inputs.iter().any(|input| reference(input) != wrong(input));
    assert!(
        caught,
        "{name}: the wrong version was not caught; widen the inputs"
    );
}

/// A buffer standing in for a span of the original's address space.
///
/// Its first byte is aligned to 8, so a rewrite may read any naturally
/// aligned word through it, and `base` must be a multiple of 8 so that
/// file addresses keep their alignment.
#[derive(Clone)]
pub struct VaImage {
    base: u32,
    storage: Vec<u8>,
    offset: usize,
    len: usize,
}

impl VaImage {
    /// A zeroed image of `len` bytes for the addresses from `base`.
    ///
    /// # Panics
    ///
    /// When `base` is not a multiple of 8, or `base + len` passes 2^32.
    #[must_use]
    pub fn new(base: u32, len: usize) -> Self {
        assert!(base.is_multiple_of(8), "image base must be 8-aligned");
        assert!(
            u64::from(base) + len as u64 <= 1 << 32,
            "image must end inside the 32-bit space"
        );
        let storage = vec![0u8; len + 8];
        let offset = storage.as_ptr().align_offset(8);
        Self {
            base,
            storage,
            offset,
            len,
        }
    }

    /// First address backed.
    #[must_use]
    pub const fn base(&self) -> u32 {
        self.base
    }

    /// Number of bytes backed.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// True when nothing is backed.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// The backed bytes.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.storage[self.offset..self.offset + self.len]
    }

    /// The backed bytes, writable.
    pub fn bytes_mut(&mut self) -> &mut [u8] {
        &mut self.storage[self.offset..self.offset + self.len]
    }

    /// Raw pointer to the first backed byte, for the runtime to hand to
    /// rewrites. Taken fresh before every call.
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.storage.as_mut_ptr().wrapping_add(self.offset)
    }

    /// The bytes of the given `(address, length)` regions, concatenated,
    /// for comparing two images over what a state models.
    ///
    /// # Panics
    ///
    /// When a region is outside the image.
    #[must_use]
    pub fn regions(&self, regions: &[(u32, u32)]) -> Vec<u8> {
        let mut out = Vec::new();
        for &(addr, len) in regions {
            let start = (addr - self.base) as usize;
            out.extend_from_slice(&self.bytes()[start..start + len as usize]);
        }
        out
    }

    /// Fills every byte with a pattern derived from `seed`, standing in for
    /// memory the test does not model.
    pub fn fill_pattern(&mut self, seed: u64) {
        let mut rng = Rng::new(seed);
        for chunk in self.bytes_mut().chunks_mut(8) {
            let word = rng.next_u64().to_le_bytes();
            chunk.copy_from_slice(&word[..chunk.len()]);
        }
    }

    fn span(&self, addr: u32, len: usize) -> Result<core::ops::Range<usize>, BoundaryError> {
        let outside = BoundaryError::OutsideImage { addr, len };
        let start = addr.checked_sub(self.base).ok_or(outside)? as usize;
        let end = start.checked_add(len).ok_or(outside)?;
        if end > self.len {
            return Err(outside);
        }
        Ok(start..end)
    }
}

impl Image32 for VaImage {
    fn read_bytes(&self, addr: u32, out: &mut [u8]) -> Result<(), BoundaryError> {
        let span = self.span(addr, out.len())?;
        out.copy_from_slice(&self.bytes()[span]);
        Ok(())
    }

    fn write_bytes(&mut self, addr: u32, bytes: &[u8]) -> Result<(), BoundaryError> {
        let span = self.span(addr, bytes.len())?;
        self.bytes_mut()[span].copy_from_slice(bytes);
        Ok(())
    }
}

impl fmt::Debug for VaImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VaImage({:#010x}, {} bytes)", self.base, self.len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rng_is_deterministic() {
        let a: Vec<u32> = inputs(7, 5, Rng::u32);
        let b: Vec<u32> = inputs(7, 5, Rng::u32);
        let c: Vec<u32> = inputs(8, 5, Rng::u32);
        assert_eq!(a, b);
        assert_ne!(a, c);
        let mut rng = Rng::new(1);
        assert!((0..1000).all(|_| rng.below(10) < 10));
    }

    #[test]
    fn float_wrappers_treat_nans_alike() {
        assert_eq!(F32(f32::from_bits(0x7FC0_0001)), F32(f32::NAN));
        assert_ne!(F32(0.0), F32(-0.0));
        assert_eq!(F64(f64::NAN), F64(-f64::NAN));
        assert_ne!(F64(1.0), F64(1.0 + f64::EPSILON));
    }

    #[test]
    fn check_accepts_equal_and_catches_wrong() {
        let ins = [1u32, 2, 3];
        check(
            "double",
            &ins,
            |x| x * 2,
            |x| x + x,
            |x| x * 2 + u32::from(*x == 3),
        );
    }

    #[test]
    #[should_panic(expected = "lifted form differs")]
    fn check_rejects_a_differing_lift() {
        check("bad", &[1u32, 2], |x| *x, |x| x + 1, |x| x + 1);
    }

    #[test]
    #[should_panic(expected = "wrong version was not caught")]
    fn check_rejects_an_uncaught_wrong_version() {
        check("blind", &[1u32, 2], |x| *x, |x| *x, |x| *x);
    }

    #[test]
    fn image_is_aligned_and_bounded() {
        let mut image = VaImage::new(0x1000, 64);
        assert_eq!(image.as_mut_ptr().align_offset(8), 0);
        image.write(0x1004, &0xAABB_CCDDu32).unwrap();
        assert_eq!(image.read::<u32>(0x1004), Ok(0xAABB_CCDD));
        assert!(image.read::<u32>(0x103E).is_err());
        assert!(image.read::<u8>(0xFFF).is_err());
        assert_eq!(
            image.regions(&[(0x1004, 2), (0x1006, 2)]),
            [0xDD, 0xCC, 0xBB, 0xAA]
        );
        let mut other = image.clone();
        other.fill_pattern(3);
        assert_ne!(other.bytes(), image.bytes());
    }
}
