//! Differential cases: the lifted ratio routine against its verified rewrites.
//!
//! The lifted routine is `lf_vehicles::veh_ratio::RatioCell::refresh`. Each
//! instance in this crate is one verified rewrite that stores the quotient of
//! two globals in a third. Every case plants the three globals, runs the
//! rewrite and the lift on the same source bits, and compares every effect:
//! the stored quotient, both sources (which the rewrite must leave alone), and
//! the rewrite's return where it has one. The dead integer return is checked
//! to be zero and is not compared further. Floats are compared as bits, so the
//! sign and payload of a not-a-number count.
//!
//! Each instance also runs two deliberately wrong lifts, which must be caught:
//! one with the operands swapped, and one that writes its quotient into the
//! numerator global instead of the destination.
//!
//! The rewrites take real addresses, so the cases run on the 32-bit target
//! only. Every loop is bounded by the case count below.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks: raw addresses flow through them
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_ratiodiff_b::rewrites;
    use lf_ratiodiff_b::rt;
    use lf_vehicles::veh_ratio::RatioCell;
    use std::sync::Mutex;
    use std::sync::MutexGuard;

    /// Cases per instance: the fixed edge pairs first, then seeded cases.
    const CASES_PER_INSTANCE: u32 = 1000;

    /// The bits planted in the destination before each run, so a rewrite
    /// that never stores is caught as a mismatch.
    const SENTINEL: u32 = 0xDEAD_BEEF;

    /// One lock per test binary: the runtime's slots are plain words behind
    /// stable addresses, so rewrite calls must not interleave.
    static TEST_LOCK: Mutex<()> = Mutex::new(());

    fn lock() -> MutexGuard<'static, ()> {
        TEST_LOCK.lock().unwrap_or_else(|poison| poison.into_inner())
    }

    /// A rewrite with no return channel.
    type UnitFn = extern "cdecl" fn();
    /// A rewrite answering the dead constant zero.
    type WordFn = extern "cdecl" fn() -> u32;
    /// A rewrite returning the quotient it stored.
    type FloatFn = extern "cdecl" fn() -> f32;

    /// The three return shapes the family comes in.
    #[derive(Clone, Copy)]
    pub enum RefreshFn {
        /// No return channel.
        Unit(UnitFn),
        /// Dead integer zero return.
        Word(WordFn),
        /// The stored quotient, as a return.
        Float(FloatFn),
    }

    /// One instance: its rewrite, its image address, and its three globals.
    pub struct Instance {
        /// The verified routine's name, for failure messages.
        pub name: &'static str,
        /// Its image address, for failure messages.
        pub address: &'static str,
        /// The rewrite to call.
        pub call: RefreshFn,
        /// The numerator global (file address).
        pub num: u32,
        /// The denominator global (file address).
        pub den: u32,
        /// The quotient global (file address).
        pub dst: u32,
    }

    include!("support/instances_gen.rs");

    /// Every effect of one run, as bits. The return is `None` for the unit
    /// shape and for the dead zero, which is checked separately.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Effects {
        ret: Option<u32>,
        num_after: u32,
        den_after: u32,
        dst_after: u32,
    }

    /// Edge pairs: zeros of both signs, infinities, quiet and signalling
    /// not-a-numbers with payloads, subnormals, extremes, and asymmetric
    /// pairs that the swapped-operand lift must get wrong. Each pair is
    /// (numerator bits, denominator bits).
    const EDGE_PAIRS: &[(u32, u32)] = &[
        (0x0000_0000, 0x3F80_0000), // +0 / 1
        (0x8000_0000, 0x3F80_0000), // -0 / 1
        (0x3F80_0000, 0x0000_0000), // 1 / +0: +inf
        (0x3F80_0000, 0x8000_0000), // 1 / -0: -inf
        (0x0000_0000, 0x0000_0000), // +0 / +0: NaN
        (0x8000_0000, 0x0000_0000), // -0 / +0: NaN
        (0x8000_0000, 0x8000_0000), // -0 / -0: NaN
        (0x7F80_0000, 0x4000_0000), // inf / 2
        (0x4000_0000, 0x7F80_0000), // 2 / inf: +0
        (0x7F80_0000, 0x7F80_0000), // inf / inf: NaN
        (0xFF80_0000, 0x7F80_0000), // -inf / inf: NaN
        (0xFF80_0000, 0x0000_0000), // -inf / +0: -inf
        (0x7FC0_0000, 0x3F80_0000), // quiet NaN / 1
        (0x3F80_0000, 0x7FC0_0000), // 1 / quiet NaN
        (0x7FC0_0001, 0x7FC0_0002), // NaN payloads: which one propagates
        (0xFFC0_0000, 0x3F80_0000), // negative quiet NaN / 1
        (0x7F80_0001, 0x3F80_0000), // signalling NaN / 1
        (0x3F80_0000, 0x7F80_0001), // 1 / signalling NaN
        (0x0000_0001, 0x3F80_0000), // smallest subnormal / 1
        (0x0000_0001, 0x0000_0001), // smallest subnormal over itself
        (0x007F_FFFF, 0x3F80_0000), // largest subnormal / 1
        (0x7F7F_FFFF, 0x7F7F_FFFF), // largest finite over itself
        (0x3F80_0000, 0x7F7F_FFFF), // 1 / largest: subnormal result
        (0x7F7F_FFFF, 0x0000_0001), // largest / smallest: overflow to inf
        (0x0080_0000, 0x0080_0000), // smallest normal over itself
        (0x4000_0000, 0x3F80_0000), // 2 / 1: swapped lift gives 0.5
        (0x4480_0000, 0x4440_0000), // 1024 / 768
        (0x3F80_0000, 0x4000_0000), // 1 / 2 = 0.5
        (0xC000_0000, 0x4000_0000), // -2 / 2 = -1
        (0x3F80_0000, 0x3F80_0000), // 1 / 1: symmetric, swapped lift agrees
    ];

    /// Special operand bit patterns mixed into the seeded cases.
    const SPECIAL: &[u32] = &[
        0x0000_0000, 0x8000_0000, 0x3F80_0000, 0xBF80_0000, 0x7F80_0000,
        0xFF80_0000, 0x7FC0_0000, 0xFFC0_0001, 0x7F80_0001, 0x0000_0001,
        0x007F_FFFF, 0x7F7F_FFFF, 0x0080_0000, 0x4000_0000, 0x3F00_0000,
    ];

    /// Seeded xorshift32: deterministic across runs.
    struct Rng(u32);

    impl Rng {
        fn next(&mut self) -> u32 {
            let mut x = self.0;
            x ^= x << 13;
            x ^= x >> 17;
            x ^= x << 5;
            self.0 = x;
            x
        }

        /// A bit pattern: a special value, a finite normal, or any word.
        fn operand(&mut self) -> u32 {
            match self.next() % 4 {
                0 => SPECIAL[(self.next() as usize) % SPECIAL.len()],
                1 => {
                    let exponent = 1 + self.next() % 254;
                    (self.next() & 0x807F_FFFF) | (exponent << 23)
                }
                _ => self.next(),
            }
        }
    }

    /// Runs the rewrite once with the given source bits.
    fn run_rewrite(inst: &Instance, nbits: u32, dbits: u32) -> Effects {
        rt::plant(inst.num, nbits);
        rt::plant(inst.den, dbits);
        rt::plant(inst.dst, SENTINEL);
        let ret = match inst.call {
            RefreshFn::Unit(f) => {
                unsafe { f() };
                None
            }
            RefreshFn::Word(f) => {
                let word = unsafe { f() };
                assert_eq!(word, 0, "{}: dead return was {word:#x}", inst.name);
                None
            }
            RefreshFn::Float(f) => Some(unsafe { f() }.to_bits()),
        };
        Effects {
            ret,
            num_after: rt::read_back(inst.num),
            den_after: rt::read_back(inst.den),
            dst_after: rt::read_back(inst.dst),
        }
    }

    /// Runs the lifted routine once with the same source bits.
    fn run_lift(nbits: u32, dbits: u32, has_return: bool) -> Effects {
        let mut cell = RatioCell::new(
            f32::from_bits(nbits),
            f32::from_bits(dbits),
            f32::from_bits(SENTINEL),
        );
        cell.refresh();
        let quotient = cell.value().to_bits();
        Effects {
            ret: has_return.then_some(quotient),
            num_after: cell.numer().to_bits(),
            den_after: cell.denom().to_bits(),
            dst_after: quotient,
        }
    }

    /// The deliberately wrong lifts. Each returns the effects it would have
    /// produced for the same inputs.
    fn wrong_swapped(nbits: u32, dbits: u32, has_return: bool) -> Effects {
        // Operands swapped: the denominator over the numerator.
        let swapped = core::hint::black_box(f32::from_bits(dbits))
            / core::hint::black_box(f32::from_bits(nbits));
        let quotient = swapped.to_bits();
        Effects {
            ret: has_return.then_some(quotient),
            num_after: nbits,
            den_after: dbits,
            dst_after: quotient,
        }
    }

    fn wrong_destination(nbits: u32, dbits: u32, has_return: bool) -> Effects {
        let mut cell = RatioCell::new(
            f32::from_bits(nbits),
            f32::from_bits(dbits),
            f32::from_bits(SENTINEL),
        );
        cell.refresh();
        let quotient = cell.value().to_bits();
        // The quotient goes into the numerator global; the destination keeps
        // its sentinel.
        Effects {
            ret: has_return.then_some(quotient),
            num_after: quotient,
            den_after: dbits,
            dst_after: SENTINEL,
        }
    }

    /// The outcome of one instance over all its cases.
    struct Outcome {
        cases: u32,
        mismatches: u32,
        first_mismatch: Option<String>,
        swapped_caught: u32,
        destination_caught: u32,
    }

    fn check_instance(inst: &Instance, seed: u32) -> Outcome {
        let has_return = !matches!(inst.call, RefreshFn::Unit(_));
        let mut rng = Rng(seed);
        let mut outcome = Outcome {
            cases: 0,
            mismatches: 0,
            first_mismatch: None,
            swapped_caught: 0,
            destination_caught: 0,
        };
        let edge = EDGE_PAIRS.iter().copied();
        let seeded = (0..CASES_PER_INSTANCE - EDGE_PAIRS.len() as u32)
            .map(|_| (rng.operand(), rng.operand()));
        for (nbits, dbits) in edge.chain(seeded) {
            outcome.cases += 1;
            let rewrite = run_rewrite(inst, nbits, dbits);
            let lift = run_lift(nbits, dbits, has_return);
            if rewrite != lift {
                outcome.mismatches += 1;
                if outcome.first_mismatch.is_none() {
                    outcome.first_mismatch = Some(format!(
                        "{} ({}): numer={nbits:#010x} denom={dbits:#010x} rewrite={rewrite:?} lift={lift:?}",
                        inst.name, inst.address
                    ));
                }
            }
            if wrong_swapped(nbits, dbits, has_return) != rewrite {
                outcome.swapped_caught += 1;
            }
            if wrong_destination(nbits, dbits, has_return) != rewrite {
                outcome.destination_caught += 1;
            }
        }
        outcome
    }

    fn assert_instance(index: usize, seed: u32) {
        let _guard = lock();
        let inst = &INSTANCES[index];
        let outcome = check_instance(inst, seed);
        assert!(
            outcome.cases >= 1000,
            "{}: only {} cases",
            inst.name,
            outcome.cases
        );
        assert_eq!(
            outcome.mismatches,
            0,
            "lift disagrees with its rewrite; first failing case: {}",
            outcome.first_mismatch.unwrap_or_default()
        );
        assert!(
            outcome.swapped_caught > 0,
            "{} ({}): the swapped-operand lift was never caught",
            inst.name,
            inst.address
        );
        assert!(
            outcome.destination_caught > 0,
            "{} ({}): the wrong-destination lift was never caught",
            inst.name,
            inst.address
        );
    }

    macro_rules! ratio_test {
        ($name:ident, $idx:expr, $seed:expr) => {
            #[test]
            fn $name() {
                assert_instance($idx, $seed);
            }
        };
    }

    #[test]
    fn instance_table_is_complete() {
        assert_eq!(INSTANCES.len(), 55);
        for inst in INSTANCES {
            assert_ne!(inst.num, inst.dst, "{}: destination is the numerator", inst.name);
            assert_ne!(inst.den, inst.dst, "{}: destination is the denominator", inst.name);
        }
    }

    ratio_test!(t_fn_00e66fa0, 0, 0x1000);
    ratio_test!(t_fn_00e67060, 1, 0x1001);
    ratio_test!(t_fn_00e67100, 2, 0x1002);
    ratio_test!(t_fn_00e671a0, 3, 0x1003);
    ratio_test!(t_fn_00e671c0, 4, 0x1004);
    ratio_test!(t_fn_00e67200, 5, 0x1005);
    ratio_test!(t_fn_00e67220, 6, 0x1006);
    ratio_test!(t_fn_00e67260, 7, 0x1007);
    ratio_test!(t_fn_00e67280, 8, 0x1008);
    ratio_test!(t_fn_00e673a0, 9, 0x1009);
    ratio_test!(t_fn_00e673c0, 10, 0x100A);
    ratio_test!(t_fn_00e67430, 11, 0x100B);
    ratio_test!(t_fn_00e67520, 12, 0x100C);
    ratio_test!(t_fn_00e67540, 13, 0x100D);
    ratio_test!(t_fn_00e67560, 14, 0x100E);
    ratio_test!(t_fn_00e675e0, 15, 0x100F);
    ratio_test!(t_fn_00e697e0, 16, 0x1010);
    ratio_test!(t_fn_00e69830, 17, 0x1011);
    ratio_test!(t_fn_00e69850, 18, 0x1012);
    ratio_test!(t_fn_00e698d0, 19, 0x1013);
    ratio_test!(t_fn_00e698f0, 20, 0x1014);
    ratio_test!(t_fn_00e69910, 21, 0x1015);
    ratio_test!(t_fn_00e69930, 22, 0x1016);
    ratio_test!(t_fn_00e69970, 23, 0x1017);
    ratio_test!(t_fn_00e699e0, 24, 0x1018);
    ratio_test!(t_fn_00e69a00, 25, 0x1019);
    ratio_test!(t_fn_00e69aa0, 26, 0x101A);
    ratio_test!(t_fn_00e69ae0, 27, 0x101B);
    ratio_test!(t_fn_00e69b00, 28, 0x101C);
    ratio_test!(t_fn_00e69b20, 29, 0x101D);
    ratio_test!(t_fn_00e69b40, 30, 0x101E);
    ratio_test!(t_fn_00e69c20, 31, 0x101F);
    ratio_test!(t_fn_00e69c60, 32, 0x1020);
    ratio_test!(t_fn_00e69c80, 33, 0x1021);
    ratio_test!(t_fn_00e69ca0, 34, 0x1022);
    ratio_test!(t_fn_00e69cc0, 35, 0x1023);
    ratio_test!(t_fn_00e69ce0, 36, 0x1024);
    ratio_test!(t_fn_00e69d00, 37, 0x1025);
    ratio_test!(t_fn_00e69d20, 38, 0x1026);
    ratio_test!(t_fn_00e69d40, 39, 0x1027);
    ratio_test!(t_fn_00e69d60, 40, 0x1028);
    ratio_test!(t_fn_00e69d80, 41, 0x1029);
    ratio_test!(t_fn_00e69da0, 42, 0x102A);
    ratio_test!(t_fn_00e69dc0, 43, 0x102B);
    ratio_test!(t_fn_00e69de0, 44, 0x102C);
    ratio_test!(t_fn_00e69e00, 45, 0x102D);
    ratio_test!(t_fn_00e69e20, 46, 0x102E);
    ratio_test!(t_fn_00e69e40, 47, 0x102F);
    ratio_test!(t_fn_00e69e60, 48, 0x1030);
    ratio_test!(t_fn_00e69e80, 49, 0x1031);
    ratio_test!(t_fn_00e69eb0, 50, 0x1032);
    ratio_test!(t_fn_00e69f00, 51, 0x1033);
    ratio_test!(t_fn_00e69f20, 52, 0x1034);
    ratio_test!(t_fn_00e69f40, 53, 0x1035);
    ratio_test!(t_fn_00e6a070, 54, 0x1036);
}
