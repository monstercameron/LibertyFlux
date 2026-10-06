//! The bullet/box/extent test protocol: priming, normalisation, expansion.
//!
//! Lifted from the three verified `script_vm_pack_point_3d`,
//! `script_vm_normalize_box_corners` and `script_vm_test_expanded_bounds`
//! rewrites. The point and box tests prime through a flag-gated callee
//! first; the box test normalises each corner pair (no swap when the
//! comparison is unordered, so NaN keeps its place); the extent test
//! expands a centre by radii in single precision, in the original's
//! operand order, then normalises and presents the bounds as two
//! overlapping views. All words are single-precision bits.

// Signatures mirror the 32-bit routines' words one by one, so long
// argument lists are inherent here.
#![allow(clippy::too_many_arguments)]

use lf_core::boundary::Handle32;

/// Tag for the extent test's opaque routine word.
#[derive(Debug)]
pub struct ExtentTag;

/// The extent test's opaque routine word.
///
/// The 32-bit routine passes a relocated address its test callee uses;
/// the lift carries it without interpreting it until that callee lifts.
pub type ExtentRoutine = Handle32<ExtentTag>;

/// First trailing word of the extent test call.
pub const EXTENT_ARG_A: u32 = 0x1c;
/// Second trailing word of the extent test call.
pub const EXTENT_ARG_B: u32 = 0x0d;

/// Primes the test machinery: the flag-gated prime callee.
pub trait Prime {
    /// Runs the prime step once.
    fn prime(&mut self);
}

/// Tests a point: the point test's callee.
pub trait TestPoint {
    /// Tests `point` with (`w`, `zero`).
    fn test(&mut self, point: [u32; 3], w: u32, zero: u32);
}

/// Tests a box: the box test's callee.
pub trait TestBox {
    /// Tests the normalised (`mins`, `maxs`) corners with `zero`.
    fn test(&mut self, mins: [u32; 3], maxs: [u32; 3], zero: u32);
}

/// Tests expanded bounds: the extent test's callee.
pub trait TestViews {
    /// Tests the row and padded views with the routine word and (`a`, `b`).
    fn test(&mut self, row: [u32; 8], routine: ExtentRoutine, view: [u32; 9], a: u32, b: u32);
}

impl<F: FnMut()> Prime for F {
    fn prime(&mut self) {
        self();
    }
}

impl<F: FnMut([u32; 3], u32, u32)> TestPoint for F {
    fn test(&mut self, point: [u32; 3], w: u32, zero: u32) {
        self(point, w, zero);
    }
}

impl<F: FnMut([u32; 3], [u32; 3], u32)> TestBox for F {
    fn test(&mut self, mins: [u32; 3], maxs: [u32; 3], zero: u32) {
        self(mins, maxs, zero);
    }
}

impl<F: FnMut([u32; 8], ExtentRoutine, [u32; 9], u32, u32)> TestViews for F {
    fn test(&mut self, row: [u32; 8], routine: ExtentRoutine, view: [u32; 9], a: u32, b: u32) {
        self(row, routine, view, a, b);
    }
}

/// The area-test protocol, shared by the three test routines.
///
/// The 32-bit routines keep no state: they translate script words into
/// test calls, so the lift is a stateless translator into the test
/// traits.
#[derive(Debug, Default, Clone, Copy)]
pub struct AreaProbe;

impl AreaProbe {
    /// Whether the flag word primes: only its low byte decides.
    fn primes(flag: u32) -> bool {
        flag & 0xFF != 0
    }

    /// Normalises one corner pair: the lesser bits first.
    ///
    /// The swap runs exactly when the first component is greater; an
    /// unordered comparison (NaN on either side) keeps both in place.
    fn norm_pair(lo: u32, hi: u32) -> (u32, u32) {
        let (mut first, mut second) = (f32::from_bits(lo), f32::from_bits(hi));
        if first > second {
            core::mem::swap(&mut first, &mut second);
        }
        (first.to_bits(), second.to_bits())
    }

    /// Subtracts in the original's operand order, bit for bit.
    fn fsub(a: u32, b: u32) -> u32 {
        (f32::from_bits(core::hint::black_box(a)) - f32::from_bits(core::hint::black_box(b)))
            .to_bits()
    }

    /// Adds in the original's operand order, bit for bit.
    fn fadd(a: u32, b: u32) -> u32 {
        (f32::from_bits(core::hint::black_box(a)) + f32::from_bits(core::hint::black_box(b)))
            .to_bits()
    }

    /// Tests a point, priming first when the flag word's low byte is set.
    pub fn test_point(
        &self,
        prime: &mut impl Prime,
        test: &mut impl TestPoint,
        x: u32,
        y: u32,
        z: u32,
        w: u32,
        flag: u32,
    ) {
        if Self::primes(flag) {
            prime.prime();
        }
        test.test([x, y, z], w, 0);
    }

    /// Tests a box, priming first and normalising each corner pair.
    pub fn test_box(
        &self,
        prime: &mut impl Prime,
        test: &mut impl TestBox,
        x0: u32,
        y0: u32,
        z0: u32,
        x1: u32,
        y1: u32,
        z1: u32,
        flag: u32,
    ) {
        if Self::primes(flag) {
            prime.prime();
        }
        let (min_x, max_x) = Self::norm_pair(x0, x1);
        let (min_y, max_y) = Self::norm_pair(y0, y1);
        let (min_z, max_z) = Self::norm_pair(z0, z1);
        test.test([min_x, min_y, min_z], [max_x, max_y, max_z], 0);
    }

    /// Tests bounds expanded from a centre by radii.
    ///
    /// Each centre word expands by its radius (`centre - radius`,
    /// `centre + radius`), each pair normalises, and the six bounds
    /// travel as the row view (minima, a zero, maxima, a zero) and the
    /// padded view (a leading zero, then the row shifted by one with a
    /// trailing zero). The routine word and the two trailing words pass
    /// through to the test callee.
    pub fn test_expanded(
        &self,
        test: &mut impl TestViews,
        routine: ExtentRoutine,
        x: u32,
        y: u32,
        z: u32,
        r0: u32,
        r1: u32,
        r2: u32,
    ) {
        let (min_x, max_x) = Self::norm_pair(Self::fsub(x, r0), Self::fadd(x, r0));
        let (min_y, max_y) = Self::norm_pair(Self::fsub(y, r1), Self::fadd(y, r1));
        let (min_z, max_z) = Self::norm_pair(Self::fsub(z, r2), Self::fadd(z, r2));
        let row = [min_x, min_y, min_z, 0, max_x, max_y, max_z, 0];
        let view = [0, min_x, min_y, min_z, 0, max_x, max_y, max_z, 0];
        test.test(row, routine, view, EXTENT_ARG_A, EXTENT_ARG_B);
    }
}
