//! Differential cases: [`RatioCell::refresh`] against its verified rewrites.
//!
//! Each case plants one instance's three globals, runs the rewrite and
//! the lift on the same source bits, and compares the stored quotient
//! bit for bit plus the rewrite's return. A deliberately wrong lift
//! (swapped operands) must be caught per instance. 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_vehicles::veh_ratio::RatioCell;
    use lf_vehratiodiff_b::rewrites::*;
    use lf_vehratiodiff_b::rt;

    #[path = "../support/mod.rs"]
    mod support;
    use support::{Rng, lock};

    /// A rewrite ending without a return channel.
    type UnitFn = extern "cdecl" fn();
    /// A rewrite answering the dead constant zero.
    type WordFn = extern "cdecl" fn() -> u32;

    /// Either rewrite shape the family comes in.
    #[derive(Clone, Copy)]
    pub enum RefreshFn {
        /// No return channel.
        Unit(UnitFn),
        /// Dead zero return.
        Word(WordFn),
    }

    /// One refresh instance: its rewrite and its three globals.
    pub struct Instance {
        /// The verified routine's name, for failure messages.
        pub name: &'static str,
        /// The rewrite to call.
        pub call: RefreshFn,
        /// The numerator global.
        pub num: u32,
        /// The denominator global.
        pub den: u32,
        /// The quotient global.
        pub dst: u32,
    }

    include!("support/instances_gen.rs");

    /// Fixed edge pairs: zeros, infinities, quiet and signalling NaNs,
    /// subnormals, extremes, and asymmetric pairs the swapped-operand
    /// mutant must get wrong. Each pair is (numerator bits, denominator
    /// bits).
    const PAIRS: &[(u32, u32)] = &[
        (0x0000_0000, 0x3F80_0000), // +0 / 1 = +0
        (0x8000_0000, 0x3F80_0000), // -0 / 1 = -0
        (0x3F80_0000, 0x0000_0000), // 1 / +0 = +inf
        (0x3F80_0000, 0x8000_0000), // 1 / -0 = -inf
        (0x0000_0000, 0x0000_0000), // +0 / +0 = NaN
        (0x8000_0000, 0x0000_0000), // -0 / +0 = NaN
        (0x8000_0000, 0x8000_0000), // -0 / -0 = NaN
        (0x7F80_0000, 0x4000_0000), // inf / 2 = inf
        (0x4000_0000, 0x7F80_0000), // 2 / inf = +0
        (0x7F80_0000, 0x7F80_0000), // inf / inf = NaN
        (0xFF80_0000, 0x7F80_0000), // -inf / inf = NaN
        (0xFF80_0000, 0x0000_0000), // -inf / +0 = -inf
        (0x7FC0_0000, 0x3F80_0000), // quiet NaN / 1
        (0x3F80_0000, 0x7FC0_0000), // 1 / quiet NaN
        (0x7FC0_0001, 0x7FC0_0002), // NaN payloads propagate
        (0xFFC0_0000, 0x3F80_0000), // negative quiet NaN / 1
        (0x7F80_0001, 0x3F80_0000), // signalling NaN quiets, no trap
        (0x3F80_0000, 0x7F80_0001), // 1 / signalling NaN
        (0x0000_0001, 0x3F80_0000), // smallest subnormal / 1
        (0x0000_0001, 0x0000_0001), // smallest subnormal both = 1
        (0x007F_FFFF, 0x3F80_0000), // largest subnormal / 1
        (0x7F7F_FFFF, 0x7F7F_FFFF), // largest finite both = 1
        (0x3F80_0000, 0x7F7F_FFFF), // 1 / largest: subnormal result
        (0x7F7F_FFFF, 0x0000_0001), // largest / smallest: overflows to inf
        (0x0080_0000, 0x0080_0000), // smallest normals = 1
        (0x4000_0000, 0x3F80_0000), // 2 / 1: mutant answers 0.5
        (0x4480_0000, 0x4440_0000), // 1024 / 768: pristine pair from one doc
        (0x3F80_0000, 0x4000_0000), // 1 / 2 = 0.5
        (0xC000_0000, 0x4000_0000), // -2 / 2 = -1
        (0x3F80_0000, 0x3F80_0000), // 1 / 1: symmetric, mutant agrees
    ];

    /// Runs one instance over the edge pairs plus seeded random bit
    /// patterns. Returns (comparisons, wrong-lift catches).
    fn run_instance(inst: &Instance, seed: u32) -> (u32, u32) {
        let mut rng = Rng(seed);
        let mut cases = 0u32;
        let mut caught = 0u32;
        let mut run = |nbits: u32, dbits: u32, cases: &mut u32, caught: &mut u32| {
            rt::plant(inst.num, nbits);
            rt::plant(inst.den, dbits);
            rt::plant(inst.dst, 0xDEAD_BEEF);
            match inst.call {
                RefreshFn::Unit(f) => unsafe { f() },
                RefreshFn::Word(f) => {
                    let ret = unsafe { f() };
                    assert_eq!(ret, 0, "{}: rewrite answered {ret}", inst.name);
                }
            }
            // Raw read: the rewrite wrote through the slot address,
            // invisibly to the borrow checker.
            let got = rt::read_back(inst.dst);
            let mut cell = RatioCell::new(
                f32::from_bits(nbits),
                f32::from_bits(dbits),
                f32::from_bits(0xDEAD_BEEF),
            );
            cell.refresh();
            assert_eq!(
                got,
                cell.value().to_bits(),
                "{}: numer={nbits:#x} denom={dbits:#x}",
                inst.name
            );
            // Deliberately wrong lift: swapped operands. Must differ on
            // any asymmetric input (2/1 versus 1/2 already does).
            let wrong = core::hint::black_box(f32::from_bits(dbits))
                / core::hint::black_box(f32::from_bits(nbits));
            if wrong.to_bits() != cell.value().to_bits() {
                *caught += 1;
            }
            *cases += 1;
        };
        for &(nbits, dbits) in PAIRS {
            run(nbits, dbits, &mut cases, &mut caught);
        }
        for _ in 0..18 {
            let (nbits, dbits) = (rng.u32(), rng.u32());
            run(nbits, dbits, &mut cases, &mut caught);
        }
        (cases, caught)
    }

    macro_rules! ratio_test {
        ($name:ident, $idx:expr, $seed:expr) => {
            #[test]
            fn $name() {
                let _guard = lock();
                let (cases, caught) = run_instance(&INSTANCES[$idx], $seed);
                assert!(cases > 40, "too few comparisons ({cases})");
                assert!(
                    caught > 0,
                    "swapped-operand lift never caught ({cases} cases)"
                );
            }
        };
    }

    include!("support/cases_gen.rs");
}
