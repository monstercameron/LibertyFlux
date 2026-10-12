//! Shared flag bits, fixed-point and float idioms of the audio voices.
//!
//! Every voice class carries the same flag byte, the same sample-rate word
//! and the same level word, and the queueing and seeking methods share one
//! milli-rate scaling idiom. The helpers here restate those once, exactly
//! as the verified rewrites compute them (checked against each rewrite;
//! where two rewrites spell the same computation differently, both
//! spellings are kept and the equivalence is noted).

/// Bit 0 of the voice flag byte: the voice is stopped.
pub const FLAG_STOPPED: u8 = 0x01;
/// Bit 1 of the voice flag byte: loop-region addressing is in force.
pub const FLAG_LOOP: u8 = 0x02;
/// Bit 3 of the voice flag byte: resume (or seek-done) is pending.
pub const FLAG_RESUME: u8 = 0x08;
/// Bit 4 of the voice flag byte: the synth/buffered path is active.
pub const FLAG_SYNTH: u8 = 0x10;
/// Bit 6 of the voice flag byte: the voice is stopping.
pub const FLAG_STOPPING: u8 = 0x40;

/// The milli-rate factor every queueing method scales by (0.001).
pub const MILLI: f32 = f32::from_bits(0x3A83_126F);

/// 2^63 exactly, as a float: magnitudes at or past this (and NaN) make the
/// original's truncating store produce the indefinite value.
pub const TWO63_F: f32 = 9223372036854775808.0;

/// Low dword of the original's truncating float-to-integer store of an
/// integral float: the value truncated to 32 bits, or zero when the store
/// would have held the indefinite value (not finite or out of range).
#[must_use]
#[inline]
pub fn fistp_low(x: f32) -> u32 {
    if x.is_finite() && x < TWO63_F && x >= -TWO63_F {
        (x as i64) as u32
    } else {
        0
    }
}

/// Truncates a float to 32 bits the way the software voice's queueing
/// method stores it: out-of-range and NaN yield zero (the low word of the
/// indefinite value). No finite float falls strictly between the largest
/// storable magnitude and 2^63, so the cast below never saturates.
#[must_use]
#[inline]
pub fn trunc_f32_to_i64_lo(v: f32) -> u32 {
    if v.is_nan() || v >= TWO63_F || v < -TWO63_F {
        0
    } else {
        (v as i64) as u32
    }
}

/// The queueing methods' scaled sample count, doubled: the low dword of
/// `floor(rate * count * 0.001)`, doubled with wraparound. `rate` and
/// `count` convert directly; the multiplies run in the original's order.
#[must_use]
#[inline]
pub fn milli_floor_doubled(rate: u32, count: u32) -> u32 {
    let scaled = (rate as f32 * count as f32) * MILLI;
    fistp_low(scaled.floor()).wrapping_mul(2)
}

/// Unsigned word to float through a double with a high-bit adjustment, as
/// the software and ADPCM queueing methods convert: bit-identical to a
/// direct conversion for every word (both round the exact value once), but
/// restated here in the rewrites' form so the proof does not depend on
/// that equivalence.
#[must_use]
#[inline]
pub fn u32_to_f32_bias(v: u32) -> f32 {
    const T_ADJ: [f64; 2] = [0.0, 4294967296.0];
    ((v as i32) as f64 + T_ADJ[(v >> 31) as usize]) as f32
}

/// The software queue's round-and-step-down sequence, which nets to the
/// floor of its input: round to nearest through the 2^23 bias (with the
/// sign folded in), then step one down when the rounded value overshoots,
/// testing overshoot as not-less-or-equal.
#[must_use]
#[inline]
pub fn round_down_magic_soft(x5: f32) -> f32 {
    const K_2P23: f32 = f32::from_bits(0x4B00_0000);
    const K_ONE: f32 = 1.0;
    let sign_bits = x5.to_bits() & 0x8000_0000;
    let sign = f32::from_bits(sign_bits);
    let x2 = if f32::from_bits(x5.to_bits() & 0x7FFF_FFFF) < K_2P23 {
        f32::from_bits(K_2P23.to_bits() | sign_bits)
    } else {
        sign
    };
    let mut x1 = x5 + x2;
    x1 -= x2;
    let x0 = x1 - x5;
    // Not-less-or-equal: true for greater-than or unordered, exactly like
    // the original's comparison.
    let step = if x0 <= sign { 0.0 } else { K_ONE };
    x1 - step
}

/// The ADPCM seek's round-and-step-down sequence: same shape as
/// [`round_down_magic_soft`], testing overshoot as greater-than. The two
/// differ only when the overshoot itself is NaN (the input is NaN or
/// infinite); both answers are NaN there and both truncate to zero, so the
/// stored words agree everywhere.
#[must_use]
#[inline]
pub fn round_down_magic_adpcm(x5: f32) -> f32 {
    const K_2P23: f32 = f32::from_bits(0x4B00_0000);
    let sign_bits = x5.to_bits() & 0x8000_0000;
    let sign = f32::from_bits(sign_bits);
    let ax = f32::from_bits(x5.to_bits() ^ sign_bits);
    let mask: u32 = if ax < K_2P23 { 0xFFFF_FFFF } else { 0 };
    let x2 = f32::from_bits((0x4B00_0000 & mask) | sign_bits);
    let mut x1 = x5 + x2;
    x1 -= x2;
    let frac = x1 - x5;
    let adjust: u32 = if frac > sign { 0xFFFF_FFFF } else { 0 };
    x1 - f32::from_bits(0x3F80_0000 & adjust)
}

/// The position methods' fixed-point combination: the frequency word
/// (16.16) scaled, halved once more after adding the play cursor, then
/// biased. Every step wraps exactly as the original's word arithmetic.
#[must_use]
#[inline]
pub fn position_arg(freq: u32, cursor: u32, bias: u32) -> u32 {
    (((freq >> 1) << 17).wrapping_add(cursor) >> 1).wrapping_add(bias)
}

/// The resume mode bit folded out of the voice flags: bit 1 or-ed with bit
/// 4 shifted down, computed by the original's shift-and-or sequence.
#[must_use]
#[inline]
pub const fn resume_mode(flags: u8) -> u32 {
    ((((flags >> 3) | flags) >> 1) & 1) as u32
}

/// The ADPCM synth path's accumulator update: from the codec table word
/// selected by the codec answer and the converted rate, the slot
/// accumulator and the predictor-table row selector. The row rounds up
/// unless the low 11 bits are exactly zero.
#[must_use]
#[inline]
pub fn synth_acc(table_word: u32, converted: u32) -> (u32, u32) {
    let edi = table_word.wrapping_mul(2) >> 2;
    let edx0 = converted.wrapping_mul(2) >> 2;
    let edx1 = (edx0 >> 11) + u32::from(edx0 & 0x7ff != 0);
    let acc = (edx1 << 11).wrapping_sub(edi);
    (acc, edx1)
}
