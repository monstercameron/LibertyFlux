//! The bar's time peers: shared time bases, samplers, the clock blend.
//!
//! Four global words feed the bar's time math as two wide/narrow pairs
//! ([`TimeBases`]); a low-byte-tested sampler ([`BaseSelect`]) picks one
//! side per use. The clock object behind the bar's link answers two
//! readings through virtual slots ([`ClockRead`]). [`blend_factors`] is
//! the one routine of the group with no `this`: a free function over a
//! clock, a scale word and the shared bases.

use lf_core::boundary::Handle32;

/// Identity of the bar's clock object (virtual slots `+0x20`/`+0x24`).
///
/// The clock layout is not lifted yet, so clocks travel as opaque handles
/// until it is.
#[derive(Debug)]
pub struct ClockTag;

/// An opaque clock object: the bar link's target.
pub type ClockHandle = Handle32<ClockTag>;

/// The four shared time-base words both sides read.
///
/// Each pair holds a narrow value (used when the selector answers false)
/// and a wide value (used when it answers true). The words convert as
/// signed on every use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeBases {
    /// The numerator pair's narrow word.
    pub num_narrow: u32,
    /// The numerator pair's wide word.
    pub num_wide: u32,
    /// The denominator pair's narrow word.
    pub den_narrow: u32,
    /// The denominator pair's wide word.
    pub den_wide: u32,
}

/// Picks one side of a time-base pair: the mode sampler.
///
/// The 32-bit sampler answers a word of which only the low byte decides;
/// the lift takes the decided bool (the proof scripts words whose full
/// value and low byte disagree).
pub trait BaseSelect {
    /// Whether the wide word wins for this use.
    fn wide(&mut self) -> bool;
}

impl<F: FnMut() -> bool> BaseSelect for F {
    fn wide(&mut self) -> bool {
        self()
    }
}

/// Reads the clock's two values: virtual slots `+0x20` and `+0x24`.
pub trait ClockRead {
    /// The slot `+0x20` reading.
    fn clock_a(&mut self) -> u32;
    /// The slot `+0x24` reading.
    fn clock_b(&mut self) -> u32;
}

/// Blends the two clock readings with the selected time bases.
///
/// Reads both clock values as signed-then-float into the answer's first
/// two components, then blends `(first / second) * scale`. When the base
/// ratio (wide/narrow numerator over wide/narrow denominator, each picked
/// by one selector sample) is below 1 the blend is further scaled by the
/// ratio; otherwise the scale itself is divided by the ratio instead. All
/// arithmetic runs in single precision in the 32-bit order, and the
/// `1.0 > ratio` test keeps its NaN path (unordered ratios divide the
/// scale).
///
/// Answers `(first, second, blend)`; `scale` is rewritten only on the
/// divide path. The 32-bit form also answers the scale pointer, which
/// carries no meaning (the proof pins it).
pub fn blend_factors(
    clock: &mut impl ClockRead,
    scale: &mut f32,
    bases: &TimeBases,
    select: &mut impl BaseSelect,
) -> (f32, f32, f32) {
    let f1 = clock.clock_a().cast_signed() as f32;
    let f2 = clock.clock_b().cast_signed() as f32;
    let num = if select.wide() {
        bases.num_wide
    } else {
        bases.num_narrow
    };
    let den = if select.wide() {
        bases.den_wide
    } else {
        bases.den_narrow
    };
    let mut x = f1 / f2;
    let ratio = (num.cast_signed() as f32) / (den.cast_signed() as f32);
    let sc = *scale;
    // Both multiplies keep the running value in the destination lane:
    // NaN payloads observe the order (the backend otherwise folds the
    // spilled running value in as the source), so each side is pinned.
    x = core::hint::black_box(x) * core::hint::black_box(sc);
    let blend = x;
    if 1.0 > ratio {
        x = core::hint::black_box(x) * core::hint::black_box(ratio);
        (f1, f2, x)
    } else {
        *scale = sc / ratio;
        (f1, f2, blend)
    }
}
