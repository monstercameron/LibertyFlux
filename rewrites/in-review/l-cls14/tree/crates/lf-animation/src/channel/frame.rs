//! The channel interface and the frame arithmetic every sampler shares.
//!
//! All animated channels turn a frame number into a key index by the same
//! two patterns (restated from the verified rewrites, which the checker
//! proved against the original):
//!
//! - Float and vector channels round down to the key at or below the frame
//!   ([`key_below`]) and snap to a key when the fraction is within
//!   [`SNAP_LO`] of one, else interpolate.
//! - Integer and boolean channels round half up ([`round_half_up`]) and
//!   take that key.
//!
//! The threshold and rounding constants below are the shared constants the
//! original reads; their values were measured from the executable's data
//! and are pinned here by their exact bit patterns.

/// Half a frame: integer and boolean channels add this, then truncate.
pub const ROUND_HALF: f32 = f32::from_bits(0x3F00_0000);

/// Rounding bias: 2^23, the float channels' round-to-nearest pivot.
pub const ROUND_MAGIC: f32 = f32::from_bits(0x4B00_0000);

/// One, as the samplers read it.
pub const ONE: f32 = f32::from_bits(0x3F80_0000);

/// Fractions at or below this snap down to the key below.
pub const SNAP_LO: f32 = f32::from_bits(0x3A83_126F);

/// Fractions above this snap up to the key above.
pub const SNAP_HI: f32 = f32::from_bits(0x3F7F_BE77);

/// 2^31 exactly: magnitudes at or past this (and NaN) truncate to the
/// conversion's indefinite value.
const I32_LIMIT: f32 = f32::from_bits(0x4F00_0000);

/// Truncates a float to an integer exactly like the original's
/// float-to-integer conversion: toward zero, with `i32::MIN` (the
/// conversion's indefinite value) for NaN, infinities and out-of-range
/// magnitudes.
#[must_use]
pub fn truncate_to_i32(x: f32) -> i32 {
    // The boundary magnitude is exact in f32, so the three-way split
    // matches the hardware: -2^31 itself converts (to `i32::MIN`, which
    // the indefinite value equals by luck), anything further out, and NaN
    // and infinities, are indefinite.
    if x.is_nan() || x >= I32_LIMIT || x < -I32_LIMIT {
        i32::MIN
    } else {
        // `x.trunc() as i32`: in range, so the cast is exact, never saturating.
        x.trunc() as i32
    }
}

/// Rounds half up: `trunc(frame + 0.5)` with the shared half constant.
#[must_use]
pub fn round_half_up(frame: f32) -> i32 {
    truncate_to_i32(frame + ROUND_HALF)
}

/// Splits a frame number into the key at or below it and the fraction
/// above that key, by the float channels' branch-free sequence: round to
/// nearest through the 2^23 bias (with the sign folded in), step one down
/// when the rounded value overshoots, truncate, and subtract back.
///
/// Every operation below is in the original's order; the fraction is in
/// `[0, 1]` for ordinary frames.
#[must_use]
pub fn key_below(frame: f32) -> (i32, f32) {
    let sign_bits = frame.to_bits() & 0x8000_0000;
    let abs = f32::from_bits(frame.to_bits() ^ sign_bits);
    let bias = f32::from_bits(
        (if abs < ROUND_MAGIC {
            ROUND_MAGIC.to_bits()
        } else {
            0
        }) | sign_bits,
    );
    let rounded = (frame + bias) - bias;
    let over = rounded - frame;
    let sign = f32::from_bits(sign_bits);
    let adjust = if over < sign { 0.0 } else { ONE };
    let index = truncate_to_i32(rounded - adjust);
    // `index as f32` rounds exactly like the original's conversion.
    let frac = frame - index as f32;
    (index, frac)
}

/// Clamps a key index into `[0, last]` with the samplers' branch shape
/// (below first, then above).
#[must_use]
pub fn clamp_index(index: i32, last: i32) -> i32 {
    if index < 0 {
        0
    } else if index > last {
        last
    } else {
        index
    }
}

/// The animation channel interface: what every channel decoder does.
///
/// `Sample` is one decoded value: a float, an integer word, a boolean
/// byte, or a padded vector. Sampling writes through `out` rather than
/// returning because some paths leave part of the output untouched (the
/// vector lerp writes x, y, z but not the padding word), and the proof
/// compares every byte.
///
/// Only channels whose at-frame sampler is verified implement this. The
/// static integer and static vector channels have no verified sampler
/// yet (the vector's frame sampler is unverified), so they expose
/// inherent methods only.
pub trait AnimChannel {
    /// One decoded sample.
    type Sample: Copy + Default;

    /// Decodes the value at frame `frame` into `out`.
    fn sample_into(&self, frame: f32, out: &mut Self::Sample);

    /// Number of stored keys.
    fn key_count(&self) -> usize;

    /// Decodes the value at frame `frame`.
    fn sample(&self, frame: f32) -> Self::Sample {
        let mut out = Self::Sample::default();
        self.sample_into(frame, &mut out);
        out
    }
}
