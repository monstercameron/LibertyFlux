//! Pure functions: code maps, classifiers and arithmetic.
//!
//! Every function here reads only its arguments and touches no memory, no
//! globals and no callees, so the lift is a straight restatement in
//! idiomatic Rust with the original's argument narrowing moved to the
//! boundary (a byte argument is a `u8`, a flag argument a `bool`, a signed
//! comparison an `i32`). Where the original's return carries more than its
//! meaning (residue above a byte result, an address instead of an index),
//! the item says what was narrowed and the differential test pins the
//! original's shape.
//!
//! Each item names its verified rewrite (`rw_*`, in `rewrites/verified/`)
//! and the original's address, so the differential harness and the
//! registry can find it.

// The conversions below are the original's own (integer to float, float
// to integer, signed reinterpretation); each is documented where it is
// made, and the differential tests pin them.
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]

/// One thousandth, the scale both product and clock conversions use.
pub const MILLI: f32 = 0.001;

/// Truncates toward zero with the x86 rule for unrepresentable values:
/// NaN, infinities and anything outside `i32`'s range give `i32::MIN`
/// (Rust's `as` would saturate and send NaN to zero instead).
///
/// Shared by every lifted function whose original converts with a
/// truncating scalar conversion.
#[must_use]
pub fn truncate_to_i32(value: f32) -> i32 {
    const LIMIT: f32 = 2_147_483_648.0;
    // A NaN is outside every range, so it takes the second branch too.
    if (-LIMIT..LIMIT).contains(&value) {
        value as i32
    } else {
        i32::MIN
    }
}

/// `floor(a * b * 0.001)` as an unsigned word.
///
/// Both factors convert to `f32` (rounding), the product is formed first
/// and then scaled, exactly in that order. The floor is taken in float and
/// the result truncated through 64 bits to the low 32, so products above
/// `u32::MAX` wrap as the original's do. The product is never negative or
/// NaN, and is finite for every pair of `u32` inputs.
///
/// Original: `rw_0088bcd0` (0x0088BCD0, `audio_scaled_product_to_int`).
#[must_use]
pub fn scaled_product_floor(a: u32, b: u32) -> u32 {
    let scaled = (a as f32) * (b as f32) * MILLI;
    scaled.floor() as i64 as u32
}

/// Rounds to the nearest `i32`, halves away from zero.
///
/// Biases by `-0.5` for negative inputs and `+0.5` otherwise (NaN takes
/// the `+0.5` side), then truncates with [`truncate_to_i32`], so NaN,
/// infinities and out-of-range values give `i32::MIN`.
///
/// Original: `rw_008a6a00` (0x008A6A00, `audio_round_half_away_to_i32`).
#[must_use]
pub fn round_half_away(value: f32) -> i32 {
    const HALF: f32 = 0.5;
    if value < 0.0 {
        truncate_to_i32(value - HALF)
    } else {
        truncate_to_i32(value + HALF)
    }
}

/// Clamped linear interpolation from `from` towards `to`.
///
/// The factor is `(x - lo) / (hi - lo)`, forced to 0 when `lo >= x` and to
/// 1 when `x >= hi`; the result is `(to - from) * factor + from`, in that
/// operation order. A NaN in either comparison falls through to the
/// division, as in the original.
///
/// Original: `rw_008aadb0` (0x008AADB0, `audio_lerp_clamped`); argument
/// order `(from, to, lo, hi, x)`.
#[must_use]
pub fn lerp_clamped(from: f32, to: f32, lo: f32, hi: f32, x: f32) -> f32 {
    let factor = if lo >= x {
        0.0
    } else if x >= hi {
        1.0
    } else {
        (x - lo) / (hi - lo)
    };
    (to - from) * factor + from
}

/// Maps `x` from `[lo, hi]` onto `[from, to]`, clamping at both ends.
///
/// Returns `from` when `lo > x`, `to` when `x > hi`, otherwise
/// `(x - lo) / (hi - lo) * (to - from) + from` in that order. NaN inputs
/// fall through the ordered comparisons to the blend.
///
/// Original: `rw_008d70e0` (0x008D70E0, `clamp_lerp`); argument order
/// `(x, lo, hi, from, to)`.
#[must_use]
pub fn clamp_map(x: f32, lo: f32, hi: f32, from: f32, to: f32) -> f32 {
    if lo > x {
        from
    } else if x > hi {
        to
    } else {
        (x - lo) / (hi - lo) * (to - from) + from
    }
}

/// Code for a selector: 6, 7 and 8 have their own codes, every other value
/// (5 included) gets the default `0x1E`.
///
/// Original: `rw_008D5180` (0x008D5180, `CodeForSelectorValue`).
#[must_use]
pub fn code_for_selector(selector: u32) -> u32 {
    match selector {
        6 => 0x20,
        7 => 0x1F,
        8 => 0x21,
        _ => 0x1E,
    }
}

/// Code for a selector, second table: 5, 6 and 7 have their own codes and
/// every other value gets `0x21`.
///
/// Original: `rw_00d38c10` (0x00D38C10, `selector_code_map`).
#[must_use]
pub fn selector_code(selector: u32) -> u32 {
    match selector {
        5 => 0x1E,
        6 => 0x20,
        7 => 0x1F,
        _ => 0x21,
    }
}

/// Maps one byte through the game's character table.
///
/// `0x8E..=0xCC` shift up by `0x32`; `0xCD`, `0xD3`, `0xF7` and `0xF8` map
/// to `0xFF`, `0xB9`, `0xB8` and `0xA8`; every other byte maps to itself.
///
/// Narrowed: the original reads only the low byte of its argument and
/// leaves residue in bits 8-31 of its result for bytes below `0x8E`
/// (all ones there). The lift is byte to byte; the differential test pins
/// the residue on the 32-bit side.
///
/// Original: `rw_0091b3c0` (0x0091B3C0, `char_byte_map`).
#[must_use]
pub fn map_char_byte(byte: u8) -> u8 {
    match byte {
        0x8E..=0xCC => byte + 0x32,
        0xCD => 0xFF,
        0xD3 => 0xB9,
        0xF7 => 0xB8,
        0xF8 => 0xA8,
        _ => byte,
    }
}

/// Buffer size for an element count: zero below 256, otherwise
/// `count² × 16 + 24`, wrapping modulo 2^32.
///
/// Original: `rw_00925E50` (0x00925E50, `input_size_from_count`).
#[must_use]
pub fn input_buffer_size(count: u32) -> u32 {
    const MIN_COUNT: u32 = 0x100;
    const HEADER: u32 = 0x18;
    if count < MIN_COUNT {
        0
    } else {
        (count.wrapping_mul(count) << 4).wrapping_add(HEADER)
    }
}

/// Classifies a (mode, value) pair of input flag words into 0, 1 or 2.
///
/// 0 when `value` is zero or `mode` has bit 10; otherwise, when `mode` has
/// bit 8, 2 if `value` has bit 10 else `value`'s bit 8; when `mode` lacks
/// bit 8, 2 if `value` has bit 8 or bit 10 else 1.
///
/// Original: `rw_009253F0` (0x009253F0, `input_flag_classify`).
#[must_use]
pub fn input_flag_class(mode: u32, value: u32) -> u32 {
    const BIT8: u32 = 0x100;
    const BIT10: u32 = 0x400;
    if value == 0 || mode & BIT10 != 0 {
        0
    } else if mode & BIT8 != 0 {
        if value & BIT10 != 0 {
            2
        } else {
            (value >> 8) & 1
        }
    } else if value & (BIT8 | BIT10) != 0 {
        2
    } else {
        1
    }
}

/// Number of slots the alternate bank is offset by.
pub const ALTERNATE_BANK_OFFSET: u32 = 0xC0;

/// Byte stride of one bank slot (used only at the boundary, where the
/// index becomes an address).
pub const BANK_SLOT_STRIDE: u32 = 0xC8;

/// Index of slot `index` in the primary bank, or in the alternate bank
/// (offset by [`ALTERNATE_BANK_OFFSET`] slots) when `alternate` is set.
///
/// Narrowed: the original returns the slot's address, `object + index ×
/// 0xC8`, and takes the bank as the low byte of a word. The lift returns
/// the index; the boundary applies
/// [`element_addr`](lf_core::boundary::element_addr) with
/// [`BANK_SLOT_STRIDE`].
///
/// Original: `rw_0094b8c0` (0x0094B8C0, `obj_slot_ptr`).
#[must_use]
pub fn bank_slot_index(index: u32, alternate: bool) -> u32 {
    if alternate {
        index.wrapping_add(ALTERNATE_BANK_OFFSET)
    } else {
        index
    }
}

/// Byte stride of the table whose element address the original computes
/// in `rw_009b7600`; that function lifts to plain indexing, and the
/// boundary form is [`element_addr`](lf_core::boundary::element_addr) with
/// this stride.
pub const RECORD_0X84_STRIDE: u32 = 0x84;

/// Size class for a byte count compared as signed: at least `0x780` gives
/// `0x3000`, at least `0x500` gives `0x1FA0`, anything smaller (every
/// negative count included) gives `0x1000`.
///
/// Original: `rw_00952630` (0x00952630, `size_class_picker`).
#[must_use]
pub fn size_class(count: i32) -> u32 {
    if count >= 0x780 {
        0x3000
    } else if count >= 0x500 {
        0x1FA0
    } else {
        0x1000
    }
}

/// The single-bit flag for a kind number, or zero for kinds with no bit.
///
/// Original: `rw_009529e0` (0x009529E0, `kind_to_flag_bit`).
#[must_use]
pub fn kind_flag_bit(kind: u32) -> u32 {
    match kind {
        2..=6 => 0x1,
        9..=18 => 0x2,
        0x15..=0x18 => 0x4,
        0x1B..=0x1D => 0x8,
        0x20..=0x21 => 0x10,
        0x24..=0x25 => 0x20,
        0x28..=0x67 => 0x40,
        0x6A..=0x6F => 0x80,
        0x72..=0x91 | 0x98 => 0x100,
        0x94 | 0x95 => 0x200,
        0x9C => 0x400,
        _ => 0,
    }
}

/// Record size in bytes for a stream tag; unassigned tags give zero.
///
/// The original reads only the low byte of its argument; the lift takes
/// the byte.
///
/// Original: `rw_009532a0` (0x009532A0, `char_size_class`).
#[must_use]
pub fn record_size_for_tag(tag: u8) -> u32 {
    match tag {
        0x00 | 0x03..=0x05 => 0x4,
        0x0F | 0x11 | 0x16 | 0x18 | 0x1D | 0x21 | 0x7E => 0x8,
        0x06 | 0x7F | 0x81 | 0x90 => 0xC,
        0x12
        | 0x28
        | 0x2A
        | 0x38
        | 0x3D
        | 0x48..=0x49
        | 0x4B
        | 0x72
        | 0x83..=0x84
        | 0x8A
        | 0x8C..=0x8D
        | 0x8F => 0x10,
        0x25 | 0x5E | 0x73 | 0x7D | 0x87 | 0x9B => 0x14,
        0x10 | 0x24 | 0x2B | 0x3F | 0x5F..=0x60 | 0x67 | 0x6A | 0x75 | 0x7B | 0x88 => 0x18,
        0x29 | 0x2D | 0x3E | 0x4A | 0x56 | 0x58..=0x59 | 0x6D | 0x7C | 0x85 | 0x89 => 0x1C,
        0x17
        | 0x36
        | 0x39
        | 0x42
        | 0x46
        | 0x5B..=0x5C
        | 0x62
        | 0x6E
        | 0x78..=0x79
        | 0x86
        | 0x9C..=0x9D => 0x20,
        0x15 | 0x31..=0x33 | 0x35 | 0x3B | 0x40 | 0x44 | 0x57 | 0x66 | 0x94 | 0x9A => 0x24,
        0x2C | 0x3C | 0x41 | 0x47 | 0x5A | 0x5D | 0x64 | 0x6B..=0x6C | 0x7A | 0x8B => 0x28,
        0x20 | 0x2E | 0x30 | 0x34 | 0x45 | 0x4C | 0x4E | 0x55 | 0x61 | 0x95 => 0x2C,
        0x3A | 0x43 | 0x52 | 0x74 | 0x80 => 0x30,
        0x37 | 0x53 | 0x65 | 0x6F => 0x34,
        0x54 | 0x63 => 0x38,
        0x4F | 0x51 | 0x82 => 0x3C,
        0x2F => 0x40,
        0x4D => 0x44,
        0x50 => 0x4C,
        0x02 | 0x76..=0x77 => 0x50,
        0x98 => 0x5C,
        0x0A..=0x0C => 0x78,
        0x0D..=0x0E => 0x88,
        0x91 => 0x98,
        0x8E => 0x128,
        0x1B => 0x370,
        0x1C => 0x398,
        _ => 0,
    }
}

/// Slot index for a band code, or zero for an unknown code.
///
/// Original: `rw_009535d0` (0x009535D0, `band_index_or_zero`).
#[must_use]
pub fn band_index(code: u32) -> u32 {
    match code {
        0x19 => 3,
        0x32 => 2,
        0x4B => 1,
        0x7D => 5,
        0x96 => 6,
        0xAF => 7,
        0xC8 => 8,
        0xE1 => 4,
        _ => 0,
    }
}

/// Entry limit for a bucket number, or zero when out of range.
///
/// Original: `rw_00953640` (0x00953640, `bucket_limit_or_zero`).
#[must_use]
pub fn bucket_limit(bucket: u32) -> u32 {
    match bucket {
        1 => 0xF,
        2 => 0xA,
        3 => 5,
        4 => 3,
        5 => 0x32,
        6 => 0x42,
        7 => 0x4B,
        8 => 0x63,
        _ => 0,
    }
}

/// Routing bucket (0 to 3) for an audio channel id.
///
/// Ids above 16 route to 1; 6 and 12 to 2; 7 and 13 to 3; any other id to
/// its low bit.
///
/// Original: `rw_0097b490` (0x0097B490, `audio_channel_classify`).
#[must_use]
pub fn audio_channel_bucket(id: u32) -> u32 {
    match id {
        6 | 12 => 2,
        7 | 13 => 3,
        17.. => 1,
        _ => id & 1,
    }
}

/// Float for a code: 0 gives 0.0, 7 gives 600.0, anything else 1.0.
///
/// Original: `rw_009f62c0` (0x009F62C0, `code_to_float`).
#[must_use]
pub fn code_to_float(code: u32) -> f32 {
    match code {
        0 => 0.0,
        7 => 600.0,
        _ => 1.0,
    }
}

/// True when a task kind is 4, 5 or 6.
///
/// Original: `rw_00a71cf0` (0x00A71CF0, `kind_is_4_5_6`); the original
/// returns the answer in the low byte.
#[must_use]
pub fn is_kind_4_to_6(kind: u32) -> bool {
    matches!(kind, 4..=6)
}

/// True when two ids are equal or either is the null id (zero).
///
/// Original: `rw_00ab6f50` (0x00AB6F50, `id_or_null_equal`).
#[must_use]
pub fn ids_match_or_null(a: u32, b: u32) -> bool {
    a == b || a == 0 || b == 0
}

/// Rate for a task kind: 2.0 for kinds 14, 19, 21 and 28, else 5.0.
///
/// Original: `rw_00b31650` (0x00B31650, `task_rate_for_kind`).
#[must_use]
pub fn task_rate(kind: u32) -> f32 {
    match kind {
        14 | 19 | 21 | 28 => 2.0,
        _ => 5.0,
    }
}

/// Band (1 to 4) for a float: 4 from 3.0 up, 3 from 1.5 up, 2 above 0.0,
/// otherwise 1 (NaN included).
///
/// Original: `rw_00b79210` (0x00B79210, `float_bucket_3_1p5_0`).
#[must_use]
pub fn float_band(value: f32) -> u32 {
    if value >= 3.0 {
        4
    } else if value >= 1.5 {
        3
    } else if value > 0.0 {
        2
    } else {
        1
    }
}

/// Float for an index: 2, 3 and 4 give 1.0, 2.0 and 3.0; anything else
/// gives 0.0.
///
/// Original: `rw_00b79250` (0x00B79250, `index_to_float_0_0_1_2_3`).
#[must_use]
pub fn index_to_float(index: u32) -> f32 {
    match index {
        2 => 1.0,
        3 => 2.0,
        4 => 3.0,
        _ => 0.0,
    }
}

/// True when the pair `(a, b)` is rejected.
///
/// `a == 1` accepts `b <= 1`; `a == 2` accepts `b` of 2 or 3; `a` of 3 or
/// 4 accepts `b` in `1..=3`; every other `a` rejects.
///
/// Original: `rw_00BE81D0` (0x00BE81D0, `pair_rule_check`); the original
/// returns 1 for rejected.
#[must_use]
pub fn pair_rejected(a: u32, b: u32) -> bool {
    let accepted = match a {
        1 => b <= 1,
        2 => matches!(b, 2 | 3),
        3 | 4 => matches!(b, 1..=3),
        _ => false,
    };
    !accepted
}

/// True for the render modes counted as active: 0, 1, 4 and 5.
///
/// Original: `rw_00d740a0` (0x00D740A0, `render_mode_is_active`); the
/// original returns the answer in the low byte.
#[must_use]
pub fn render_mode_is_active(mode: u32) -> bool {
    matches!(mode, 0 | 1 | 4 | 5)
}

/// True for the shadow render modes: 8, 9 and 10.
///
/// Original: `rw_00d740e0` (0x00D740E0, `render_mode_is_shadow`); the
/// original returns the answer in the low byte.
#[must_use]
pub fn render_mode_is_shadow(mode: u32) -> bool {
    matches!(mode, 8..=10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncation_follows_the_x86_rule() {
        assert_eq!(truncate_to_i32(1.9), 1);
        assert_eq!(truncate_to_i32(-1.9), -1);
        assert_eq!(truncate_to_i32(f32::NAN), i32::MIN);
        assert_eq!(truncate_to_i32(f32::INFINITY), i32::MIN);
        assert_eq!(truncate_to_i32(f32::NEG_INFINITY), i32::MIN);
        assert_eq!(truncate_to_i32(2_147_483_648.0), i32::MIN);
        assert_eq!(truncate_to_i32(-2_147_483_648.0), i32::MIN);
        assert_eq!(truncate_to_i32(2_147_483_520.0), 2_147_483_520);
    }

    #[test]
    fn product_floor_matches_hand_values() {
        assert_eq!(scaled_product_floor(0, 5), 0);
        assert_eq!(scaled_product_floor(1000, 3), 3);
        assert_eq!(scaled_product_floor(999, 1), 0);
        assert_eq!(scaled_product_floor(1500, 1), 1);
        // Large products wrap through the low 32 bits of the 64-bit floor.
        let wide = ((u32::MAX as f32) * (u32::MAX as f32) * MILLI).floor() as i64;
        assert_eq!(scaled_product_floor(u32::MAX, u32::MAX), wide as u32);
    }

    #[test]
    fn rounding_goes_away_from_zero() {
        assert_eq!(round_half_away(0.5), 1);
        assert_eq!(round_half_away(-0.5), -1);
        assert_eq!(round_half_away(2.4), 2);
        assert_eq!(round_half_away(-2.6), -3);
        assert_eq!(round_half_away(f32::NAN), i32::MIN);
    }

    #[test]
    fn interpolations_clamp_and_blend() {
        assert_eq!(
            lerp_clamped(10.0, 20.0, 0.0, 1.0, -1.0).to_bits(),
            10.0f32.to_bits()
        );
        assert_eq!(
            lerp_clamped(10.0, 20.0, 0.0, 1.0, 2.0).to_bits(),
            20.0f32.to_bits()
        );
        assert_eq!(
            lerp_clamped(10.0, 20.0, 0.0, 1.0, 0.5).to_bits(),
            15.0f32.to_bits()
        );
        assert!(lerp_clamped(10.0, 20.0, f32::NAN, 1.0, 0.5).is_nan());
        assert_eq!(
            clamp_map(-1.0, 0.0, 1.0, 3.0, 5.0).to_bits(),
            3.0f32.to_bits()
        );
        assert_eq!(
            clamp_map(9.0, 0.0, 1.0, 3.0, 5.0).to_bits(),
            5.0f32.to_bits()
        );
        assert_eq!(
            clamp_map(0.25, 0.0, 1.0, 3.0, 5.0).to_bits(),
            3.5f32.to_bits()
        );
    }

    #[test]
    fn code_maps() {
        assert_eq!(
            [5, 6, 7, 8, 9].map(code_for_selector),
            [0x1E, 0x20, 0x1F, 0x21, 0x1E]
        );
        assert_eq!([5, 6, 7, 8].map(selector_code), [0x1E, 0x20, 0x1F, 0x21]);
        assert_eq!([0x19, 0x64, 0xE1, 1].map(band_index), [3, 0, 4, 0]);
        assert_eq!([0, 1, 8, 9].map(bucket_limit), [0, 0xF, 0x63, 0]);
        assert_eq!(
            [6, 7, 12, 13, 17, 4, 5].map(audio_channel_bucket),
            [2, 3, 2, 3, 1, 0, 1]
        );
        assert_eq!(
            [0, 2, 0x28, 0x98, 0x9C, 0x9D].map(kind_flag_bit),
            [0, 1, 0x40, 0x100, 0x400, 0]
        );
        assert_eq!(
            [0x00, 0x8E, 0x1C, 0xFF].map(record_size_for_tag),
            [4, 0x128, 0x398, 0]
        );
    }

    #[test]
    fn character_table() {
        assert_eq!(map_char_byte(0x41), 0x41);
        assert_eq!(map_char_byte(0x8E), 0xC0);
        assert_eq!(map_char_byte(0xCC), 0xFE);
        assert_eq!(map_char_byte(0xCD), 0xFF);
        assert_eq!(map_char_byte(0xD3), 0xB9);
        assert_eq!(map_char_byte(0xD4), 0xD4);
        assert_eq!(map_char_byte(0xF8), 0xA8);
        assert_eq!(map_char_byte(0xF9), 0xF9);
    }

    #[test]
    fn sizes_and_classes() {
        assert_eq!(input_buffer_size(0xFF), 0);
        assert_eq!(input_buffer_size(0x100), 0x10_0000 + 0x18);
        assert_eq!(size_class(-1), 0x1000);
        assert_eq!(size_class(0x500), 0x1FA0);
        assert_eq!(size_class(0x780), 0x3000);
        assert_eq!(input_flag_class(0, 0), 0);
        assert_eq!(input_flag_class(0x400, 1), 0);
        assert_eq!(input_flag_class(0x100, 0x400), 2);
        assert_eq!(input_flag_class(0x100, 0x100), 1);
        assert_eq!(input_flag_class(0x100, 0x1), 0);
        assert_eq!(input_flag_class(0, 0x100), 2);
        assert_eq!(input_flag_class(0, 0x1), 1);
        assert_eq!(bank_slot_index(3, false), 3);
        assert_eq!(bank_slot_index(3, true), 0xC3);
    }

    #[test]
    fn predicates_and_float_tables() {
        assert!(is_kind_4_to_6(5) && !is_kind_4_to_6(7));
        assert!(ids_match_or_null(0, 9) && ids_match_or_null(4, 4) && !ids_match_or_null(4, 5));
        assert_eq!(task_rate(19).to_bits(), 2.0f32.to_bits());
        assert_eq!(task_rate(20).to_bits(), 5.0f32.to_bits());
        assert_eq!(float_band(f32::NAN), 1);
        assert_eq!(float_band(1.5), 3);
        assert_eq!(index_to_float(4).to_bits(), 3.0f32.to_bits());
        assert_eq!(code_to_float(7).to_bits(), 600.0f32.to_bits());
        assert!(!pair_rejected(1, 1) && pair_rejected(1, 2));
        assert!(!pair_rejected(3, 3) && pair_rejected(4, 0));
        assert!(pair_rejected(0, 0));
        assert!(render_mode_is_active(5) && !render_mode_is_active(2));
        assert!(render_mode_is_shadow(10) && !render_mode_is_shadow(11));
    }
}
