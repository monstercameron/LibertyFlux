//! A second writing of the four channel steps, with deliberate faults.
//!
//! The control ([`Mutant::None`]) must agree with the verified rewrites,
//! which shows this writing is faithful. Each other mutant changes one
//! thing in it; the differential cases must see every one of them on every
//! channel. The arithmetic order matches the lifted module.

use lf_math::shaft::{Calibration, Channel, Shaft, ShaftHost};

/// Which deliberate fault a writing carries.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mutant {
    /// No fault: the control.
    None,
    /// The squared length drops its last term.
    DropLengthTerm,
    /// The step answers its first seed instead of its third.
    ReturnFirstSeed,
    /// The root is taken even for a zero length (the factor becomes infinite).
    NoZeroCheck,
    /// The second mixed value reads the third table column.
    MixColumnSwap,
    /// The table product in the first mixed value takes its operands in the
    /// other order: a difference only when both are not-a-number with
    /// different payloads or signs.
    SwapTableOperands,
}

/// The faulty writings: every one must be caught on every channel.
pub const WRONG: [Mutant; 5] = [
    Mutant::DropLengthTerm,
    Mutant::ReturnFirstSeed,
    Mutant::NoZeroCheck,
    Mutant::MixColumnSwap,
    Mutant::SwapTableOperands,
];

/// What one step produces: the answer and the values the step stores.
#[derive(Clone, Copy, Debug)]
pub struct Out {
    pub ret: f32,
    pub seeds: [f32; 3],
    pub residual: f32,
    pub mixed: [f32; 3],
    pub total: f32,
}

const SIGN: u32 = 0x8000_0000;

fn neg(a: f32) -> f32 {
    f32::from_bits(a.to_bits() ^ SIGN)
}

fn mul(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}

fn add(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}

fn sub(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) - core::hint::black_box(b)
}

fn div(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) / core::hint::black_box(b)
}

fn offset(gain: f32, cal: Calibration, dt: f32) -> f32 {
    let ratio = div(dt, cal.divisor);
    let doubled = mul(gain, 2.0);
    let product = mul(doubled, cal.scale);
    let half = mul(product, 0.5);
    sub(mul(ratio, product), half)
}

fn unit_factor(host: &mut dyn ShaftHost, radicand: f32, m: Mutant) -> f32 {
    if radicand == 0.0 && m != Mutant::NoZeroCheck {
        0.0
    } else {
        div(1.0, host.root(radicand))
    }
}

/// One channel step of the writing `m` on channel `ch` (0 to 3).
pub fn step(m: Mutant, ch: usize, shaft: &Shaft, host: &mut dyn ShaftHost, dt: f32) -> Out {
    let cal = shaft.calibration[ch >> 1];
    let g = shaft.gain;
    let neg_g = neg(g);
    let mut u = offset(g, cal, dt);
    if ch >= 2 {
        u = neg(u);
    }
    let cross = if ch < 2 {
        sub(mul(neg_g, 0.0), mul(u, 0.0))
    } else {
        sub(mul(u, 0.0), mul(neg_g, 0.0))
    };
    let neg_u = if ch == 0 { mul(u, -1.0) } else { neg(u) };
    let ga = neg(neg_g);
    let sg = mul(neg_g, -1.0);
    let seeds = if shaft.seeds_fixed {
        match ch {
            0 => [1.0, 0.0, 0.0],
            1 => [-1.0, 0.0, 0.0],
            2 => [0.0, -1.0, 0.0],
            _ => [0.0, 1.0, 0.0],
        }
    } else {
        let radicand = match ch {
            0 => add(add(mul(cross, cross), mul(neg_g, neg_g)), mul(neg_u, neg_u)),
            1 => add(add(mul(cross, cross), mul(ga, ga)), mul(u, u)),
            2 => add(add(mul(sg, sg), mul(cross, cross)), mul(u, u)),
            _ => add(add(mul(neg_g, neg_g), mul(cross, cross)), mul(neg_u, neg_u)),
        };
        let radicand = if m == Mutant::DropLengthTerm {
            match ch {
                0 => add(mul(cross, cross), mul(neg_g, neg_g)),
                1 => add(mul(cross, cross), mul(ga, ga)),
                2 => add(mul(sg, sg), mul(cross, cross)),
                _ => add(mul(neg_g, neg_g), mul(cross, cross)),
            }
        } else {
            radicand
        };
        let f = unit_factor(host, radicand, m);
        let cross_seed = mul(cross, f);
        match ch {
            0 => [mul(f, neg_g), cross_seed, mul(neg_u, f)],
            1 => [mul(f, ga), cross_seed, mul(u, f)],
            2 => [mul(f, cross), mul(sg, f), mul(u, f)],
            _ => [mul(f, cross), mul(neg_g, f), mul(neg_u, f)],
        }
    };
    let residual = if ch < 2 {
        add(
            add(mul(u, seeds[0]), mul(seeds[1], 0.0)),
            mul(seeds[2], neg_g),
        )
    } else {
        add(
            add(mul(seeds[1], u), mul(seeds[0], 0.0)),
            mul(seeds[2], neg_g),
        )
    };
    let (mixed, total) = mix(m, &shaft.table, seeds, residual);
    let ret = if m == Mutant::ReturnFirstSeed {
        seeds[0]
    } else {
        seeds[2]
    };
    Out {
        ret,
        seeds,
        residual,
        mixed,
        total,
    }
}

fn mix(m: Mutant, table: &[[f32; 3]; 4], seeds: [f32; 3], residual: f32) -> ([f32; 3], f32) {
    let [a, b, c] = seeds;
    let qa = mul(residual, a);
    let qb = mul(b, residual);
    let qc = mul(c, residual);
    let sum = |j: usize| {
        add(
            add(
                add(mul(table[0][j], qa), mul(table[1][j], qb)),
                mul(table[2][j], qc),
            ),
            table[3][j],
        )
    };
    let mixed_at = |j: usize| {
        add(
            add(mul(table[0][j], a), mul(table[1][j], b)),
            mul(table[2][j], c),
        )
    };
    let first_table_product = if m == Mutant::SwapTableOperands {
        mul(a, table[0][0])
    } else {
        mul(table[0][0], a)
    };
    let sums = [sum(0), sum(1), sum(2)];
    let second_column = if m == Mutant::MixColumnSwap { 2 } else { 1 };
    let mut mixed = [mixed_at(0), mixed_at(second_column), mixed_at(2)];
    if m == Mutant::SwapTableOperands {
        mixed[0] = add(
            add(first_table_product, mul(table[1][0], b)),
            mul(table[2][0], c),
        );
    }
    let total = add(
        add(mul(mixed[1], sums[1]), mul(mixed[0], sums[0])),
        mul(mixed[2], sums[2]),
    );
    (mixed, total)
}

/// Stores a writing's output in a channel record the way the lifted object
/// does, so the same comparison serves both.
pub fn to_channel(out: &Out, dt: f32) -> Channel {
    Channel {
        input: dt,
        seeds: out.seeds,
        residual: out.residual,
        mixed: out.mixed,
        total: out.total,
    }
}
