//! The arithmetic of the four channel steps and the mix they share.
//!
//! Names follow what each quantity is, not the original's registers. For a
//! channel with gain `g`, calibration pair `c` and step length `dt`:
//!
//! - the offset `u` is `dt / divisor` times `2 g scale`, less half of
//!   `2 g scale` (channels 2 and 3 then negate it);
//! - the seeds are a direction: unless the seeds are fixed, the three
//!   quantities of the channel's pattern (see each head) are divided by
//!   the root of the sum of their squares, or are all zero when that sum is
//!   exactly zero;
//! - the residual is the offset times one seed (the first seed for channels
//!   0 and 1, the second for channels 2 and 3), plus the other of the first
//!   two seeds times zero, plus the third seed times the negated gain;
//! - the mix multiplies the residual into the seeds and combines the table's
//!   columns with them, and the total combines the three mixed values with
//!   three weighted column sums.
//!
//! Every product and sum keeps its operand order, and each operand passes
//! through [`core::hint::black_box`]: a not-a-number takes its payload and
//! sign from the first operand, so the order is part of the behaviour.
//! Negation flips the sign bit (an exact operation), as the original does.

use super::{Calibration, ShaftHost};

/// The sign bit of a single-precision float.
const SIGN_BIT: u32 = 0x8000_0000;
/// The constant two.
const TWO: f32 = 2.0;
/// The constant one half.
const HALF: f32 = 0.5;
/// The constant one.
const ONE: f32 = 1.0;
/// The constant minus one.
const NEG_ONE: f32 = -1.0;
/// Positive zero.
const ZERO: f32 = 0.0;

#[inline(always)]
fn mul(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}

#[inline(always)]
fn add(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}

#[inline(always)]
fn sub(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) - core::hint::black_box(b)
}

#[inline(always)]
fn div(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) / core::hint::black_box(b)
}

#[inline(always)]
fn neg(a: f32) -> f32 {
    f32::from_bits(a.to_bits() ^ SIGN_BIT)
}

/// What a channel's own arithmetic produces before the mix: the three seeds
/// and the residual scalar.
pub(super) struct Head {
    pub(super) seeds: [f32; 3],
    pub(super) residual: f32,
}

/// The step's offset along the channel's axis: `dt / divisor`, times
/// `2 * gain * scale`, less half of that product.
fn offset(gain: f32, calibration: Calibration, dt: f32) -> f32 {
    let ratio = div(dt, calibration.divisor);
    let doubled = mul(gain, TWO);
    let product = mul(doubled, calibration.scale);
    let half_product = mul(product, HALF);
    sub(mul(ratio, product), half_product)
}

/// The factor that makes a direction unit length: one over the root of
/// `radicand`, or zero when the radicand is exactly zero (no root is taken).
/// A not-a-number radicand takes the root, as the original does.
fn unit_factor(host: &mut impl ShaftHost, radicand: f32) -> f32 {
    if radicand == ZERO {
        ZERO
    } else {
        div(ONE, host.root(radicand))
    }
}

/// Channel 0. The offset is not negated. The seeds are the gain axis with
/// the gain negated, the cross term, and the offset axis with the offset
/// negated. Fixed, they are the unit vector on the first axis. The residual
/// is built from the offset and the first seed.
pub(super) fn head_0(
    gain: f32,
    calibration: Calibration,
    seeds_fixed: bool,
    host: &mut impl ShaftHost,
    dt: f32,
) -> Head {
    let u = offset(gain, calibration, dt);
    let neg_gain = neg(gain);
    let cross = sub(mul(neg_gain, ZERO), mul(u, ZERO));
    let seeds = if seeds_fixed {
        [ONE, ZERO, ZERO]
    } else {
        let neg_u = mul(u, NEG_ONE);
        let radicand = add(
            add(mul(cross, cross), mul(neg_gain, neg_gain)),
            mul(neg_u, neg_u),
        );
        let f = unit_factor(host, radicand);
        [mul(f, neg_gain), mul(cross, f), mul(neg_u, f)]
    };
    let residual = add(
        add(mul(u, seeds[0]), mul(seeds[1], ZERO)),
        mul(seeds[2], neg_gain),
    );
    Head { seeds, residual }
}

/// Channel 1. The offset is not negated. The seeds are the gain axis (not
/// negated), the cross term, and the offset axis. Fixed, they are minus the
/// unit vector on the first axis.
pub(super) fn head_1(
    gain: f32,
    calibration: Calibration,
    seeds_fixed: bool,
    host: &mut impl ShaftHost,
    dt: f32,
) -> Head {
    let u = offset(gain, calibration, dt);
    let neg_gain = neg(gain);
    let cross = sub(mul(neg_gain, ZERO), mul(u, ZERO));
    let seeds = if seeds_fixed {
        [NEG_ONE, ZERO, ZERO]
    } else {
        let gain_axis = neg(neg_gain);
        let radicand = add(
            add(mul(cross, cross), mul(gain_axis, gain_axis)),
            mul(u, u),
        );
        let f = unit_factor(host, radicand);
        [mul(f, gain_axis), mul(cross, f), mul(u, f)]
    };
    let residual = add(
        add(mul(u, seeds[0]), mul(seeds[1], ZERO)),
        mul(seeds[2], neg_gain),
    );
    Head { seeds, residual }
}

/// Channel 2. The offset is negated, and the cross term is built from the
/// offset first. The seeds are the cross term, the gain axis (the gain times
/// minus one) and the offset axis. Fixed, they are minus the unit vector on
/// the second axis. The residual takes the second seed against the offset.
pub(super) fn head_2(
    gain: f32,
    calibration: Calibration,
    seeds_fixed: bool,
    host: &mut impl ShaftHost,
    dt: f32,
) -> Head {
    let u = neg(offset(gain, calibration, dt));
    let neg_gain = neg(gain);
    let cross = sub(mul(u, ZERO), mul(neg_gain, ZERO));
    let scaled_gain = mul(neg_gain, NEG_ONE);
    let seeds = if seeds_fixed {
        [ZERO, NEG_ONE, ZERO]
    } else {
        let radicand = add(
            add(mul(scaled_gain, scaled_gain), mul(cross, cross)),
            mul(u, u),
        );
        let f = unit_factor(host, radicand);
        [mul(f, cross), mul(scaled_gain, f), mul(u, f)]
    };
    let residual = add(
        add(mul(seeds[1], u), mul(seeds[0], ZERO)),
        mul(seeds[2], neg_gain),
    );
    Head { seeds, residual }
}

/// Channel 3. The offset is negated, and the cross term is built from the
/// offset first. The seeds are the cross term, the gain axis (negated) and
/// the negated offset axis. Fixed, they are the unit vector on the second
/// axis. The residual takes the second seed against the offset.
pub(super) fn head_3(
    gain: f32,
    calibration: Calibration,
    seeds_fixed: bool,
    host: &mut impl ShaftHost,
    dt: f32,
) -> Head {
    let u = neg(offset(gain, calibration, dt));
    let neg_gain = neg(gain);
    let cross = sub(mul(u, ZERO), mul(neg_gain, ZERO));
    let neg_u = neg(u);
    let seeds = if seeds_fixed {
        [ZERO, ONE, ZERO]
    } else {
        let radicand = add(
            add(mul(neg_gain, neg_gain), mul(cross, cross)),
            mul(neg_u, neg_u),
        );
        let f = unit_factor(host, radicand);
        [mul(f, cross), mul(neg_gain, f), mul(neg_u, f)]
    };
    let residual = add(
        add(mul(seeds[1], u), mul(seeds[0], ZERO)),
        mul(seeds[2], neg_gain),
    );
    Head { seeds, residual }
}

/// The mix every channel shares: the residual scales the seeds, the table's
/// first three rows weight them into three mixed values, its fourth row is
/// an additive term of three weighted sums, and the total combines the
/// mixed values with those sums.
pub(super) fn mix(table: &[[f32; 3]; 4], seeds: [f32; 3], residual: f32) -> ([f32; 3], f32) {
    let [a, b, c] = seeds;
    let scaled_a = mul(residual, a);
    let scaled_b = mul(b, residual);
    let scaled_c = mul(c, residual);
    let weighted_sum = |column: usize| {
        add(
            add(
                add(
                    mul(table[0][column], scaled_a),
                    mul(table[1][column], scaled_b),
                ),
                mul(table[2][column], scaled_c),
            ),
            table[3][column],
        )
    };
    let mixed_value = |column: usize| {
        add(
            add(mul(table[0][column], a), mul(table[1][column], b)),
            mul(table[2][column], c),
        )
    };
    let sums = [weighted_sum(0), weighted_sum(1), weighted_sum(2)];
    let mixed = [mixed_value(0), mixed_value(1), mixed_value(2)];
    let total = add(
        add(mul(mixed[1], sums[1]), mul(mixed[0], sums[0])),
        mul(mixed[2], sums[2]),
    );
    (mixed, total)
}
