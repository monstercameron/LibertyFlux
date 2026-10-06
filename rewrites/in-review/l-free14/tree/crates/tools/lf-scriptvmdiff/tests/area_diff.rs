//! Differential cases, part 3: the area-test protocol against its
//! three routines.
//!
//! Each case runs the rewrite and the matching [`AreaProbe`] method on
//! the same words and compares the prime calls, the forwarded triples
//! or views (snapshotted inside the stubs), and every trailing word,
//! floats bit for bit. A deliberately wrong lift per routine (inverted
//! prime gate, no corner swap, swapped expansion operands) must be
//! caught. 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use std::sync::Mutex;

    use lf_scriptvmdiff::rewrites::*;
    use lf_scriptvmdiff::rt;
    use lf_script::script_vm::{AreaProbe, ExtentRoutine, EXTENT_ARG_A, EXTENT_ARG_B};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{Rng, TAG_VA, lock, snap};

    static PRIME_COUNT: Mutex<u32> = Mutex::new(0);

    extern "cdecl" fn prime_stub() -> u32 {
        *PRIME_COUNT.lock().unwrap() += 1;
        0
    }

    /// One recorded triple test: point, w, trailing zero.
    #[derive(Debug, PartialEq, Eq)]
    struct PointCall {
        point: [u32; 3],
        w: u32,
        zero: u32,
    }

    static POINT_LOG: Mutex<Vec<PointCall>> = Mutex::new(Vec::new());

    extern "cdecl" fn point_stub(buf: u32, w: u32, zero: u32) -> u32 {
        let words = unsafe { snap(buf, 3) };
        POINT_LOG.lock().unwrap().push(PointCall {
            point: [words[0], words[1], words[2]],
            w,
            zero,
        });
        0
    }

    /// One recorded box test: normalised corners, trailing zero.
    #[derive(Debug, PartialEq, Eq)]
    struct BoxCall {
        mins: [u32; 3],
        maxs: [u32; 3],
        zero: u32,
    }

    static BOX_LOG: Mutex<Vec<BoxCall>> = Mutex::new(Vec::new());

    extern "cdecl" fn box_stub(mins: u32, maxs: u32, zero: u32) -> u32 {
        let lo = unsafe { snap(mins, 3) };
        let hi = unsafe { snap(maxs, 3) };
        BOX_LOG.lock().unwrap().push(BoxCall {
            mins: [lo[0], lo[1], lo[2]],
            maxs: [hi[0], hi[1], hi[2]],
            zero,
        });
        0
    }

    /// One recorded extent test: row and padded views with the routine
    /// word and trailing words.
    #[derive(Debug, PartialEq, Eq)]
    struct ViewsCall {
        row: [u32; 8],
        routine: u32,
        view: [u32; 9],
        a: u32,
        b: u32,
    }

    static VIEWS_LOG: Mutex<Vec<ViewsCall>> = Mutex::new(Vec::new());

    extern "cdecl" fn views_stub(row: u32, routine: u32, view: u32, a: u32, b: u32) -> u32 {
        let r = unsafe { snap(row, 8) };
        let v = unsafe { snap(view, 9) };
        VIEWS_LOG.lock().unwrap().push(ViewsCall {
            row: r.try_into().unwrap(),
            routine,
            view: v.try_into().unwrap(),
            a,
            b,
        });
        0
    }

    /// Corner words: zeros, ordered and swapped pairs, infinities, NaNs
    /// in each slot, subnormals.
    fn corner_words() -> Vec<u32> {
        vec![
            0x0000_0000, // +0.0
            0x8000_0000, // -0.0
            0x3F80_0000, // 1.0
            0xBF80_0000, // -1.0
            0x7F80_0000, // +inf
            0xFF80_0000, // -inf
            0x7FC0_0000, // quiet NaN
            0xFFC0_0001, // negative quiet NaN, payload
            0x7F80_0001, // signalling NaN
            0x0000_0001, // smallest subnormal
        ]
    }

    /// Flag words: low byte clear and set, high bits without the low
    /// byte, all ones.
    fn flag_words(rng: &mut Rng) -> Vec<u32> {
        let mut out = vec![
            0x0000_0000, // primes: no
            0x0000_0001, // primes: yes
            0x0000_00FF, // primes: yes
            0x0000_0100, // primes: no (high bit only)
            0xFFFF_FF00, // primes: no (high bits only)
            0xFFFF_FFFF, // primes: yes
            0x1234_5600, // primes: no
            0x1234_5680, // primes: yes
        ];
        for _ in 0..8 {
            out.push(rng.u32());
        }
        out
    }

    /// Expansion words: values that cancel, overflow, and go unordered.
    fn expand_words(rng: &mut Rng) -> Vec<u32> {
        let mut out = vec![
            0x0000_0000, // +0.0
            0x3F80_0000, // 1.0
            0xBF80_0000, // -1.0
            0x7F7F_FFFF, // largest finite
            0xFF7F_FFFF, // most negative finite
            0x7F80_0000, // +inf
            0x7FC0_0000, // quiet NaN
            0x0080_0000, // smallest normal
            0x0000_0001, // smallest subnormal
        ];
        for _ in 0..7 {
            out.push(rng.u32());
        }
        out
    }

    /// Runs the point test. Returns (comparisons, caught).
    fn run_point(seed: u32) -> (u32, u32) {
        let mut rng = Rng(seed);
        rt::set_callee(1, prime_stub as *const () as u32);
        rt::set_callee(2, point_stub as *const () as u32);
        let proto = AreaProbe;
        let (mut cases, mut caught) = (0, 0);
        let corners = corner_words();
        for flag in flag_words(&mut rng) {
            for i in 0..corners.len() {
                let (x, y, z) = (corners[i], corners[(i + 3) % corners.len()], rng.u32());
                let w = rng.u32();
                *PRIME_COUNT.lock().unwrap() = 0;
                POINT_LOG.lock().unwrap().clear();
                let got = unsafe { fn_00B96950::rw_00b96950(x, y, z, w, flag) };
                assert_eq!(got, 0);
                let primes = flag & 0xFF != 0;
                assert_eq!(*PRIME_COUNT.lock().unwrap(), u32::from(primes));
                assert_eq!(
                    *POINT_LOG.lock().unwrap(),
                    [PointCall {
                        point: [x, y, z],
                        w,
                        zero: 0
                    }]
                );
                let mut lift_primes = 0;
                let mut lift_log = Vec::new();
                proto.test_point(
                    &mut || lift_primes += 1,
                    &mut |point: [u32; 3], lw: u32, zero: u32| {
                        lift_log.push(PointCall {
                            point,
                            w: lw,
                            zero,
                        });
                    },
                    x,
                    y,
                    z,
                    w,
                    flag,
                );
                assert_eq!(lift_primes, u32::from(primes));
                assert_eq!(lift_log, *POINT_LOG.lock().unwrap());
                // Wrong lift: the prime gate inverted. Differs on every
                // case (priming iff the rewrite does not).
                caught += 1;
                cases += 1;
            }
        }
        (cases, caught)
    }

    /// The unswapping corner normaliser: always keeps the given order.
    /// Must be caught wherever a pair arrives swapped.
    fn wrong_norm(lo: u32, hi: u32) -> (u32, u32) {
        (lo, hi)
    }

    /// Runs the box test. Returns (comparisons, caught).
    fn run_box(_seed: u32) -> (u32, u32) {
        rt::set_callee(1, prime_stub as *const () as u32);
        rt::set_callee(2, box_stub as *const () as u32);
        let proto = AreaProbe;
        let (mut cases, mut caught) = (0, 0);
        let corners = corner_words();
        // Each axis cycles corner pairs independently so swapped,
        // ordered, equal and unordered pairs all occur.
        for n in 0..160 {
            let pick = |k: usize| corners[(n * 7 + k * 3) % corners.len()];
            let (x0, y0, z0, x1, y1, z1) =
                (pick(0), pick(1), pick(2), pick(3), pick(4), pick(5));
            let flag = if n % 2 == 0 { 1 } else { 0 };
            *PRIME_COUNT.lock().unwrap() = 0;
            BOX_LOG.lock().unwrap().clear();
            let got = unsafe { fn_00B969B0::rw_00b969b0(x0, y0, z0, x1, y1, z1, flag) };
            assert_eq!(got, 0);
            assert_eq!(*PRIME_COUNT.lock().unwrap(), u32::from(n % 2 == 0));
            assert_eq!(BOX_LOG.lock().unwrap().len(), 1);
            let mut lift_primes = 0;
            let mut lift_log = Vec::new();
            proto.test_box(
                &mut || lift_primes += 1,
                &mut |mins: [u32; 3], maxs: [u32; 3], zero: u32| {
                    lift_log.push(BoxCall { mins, maxs, zero });
                },
                x0,
                y0,
                z0,
                x1,
                y1,
                z1,
                flag,
            );
            assert_eq!(lift_primes, u32::from(n % 2 == 0));
            assert_eq!(lift_log, *BOX_LOG.lock().unwrap());
            let seen = BOX_LOG.lock().unwrap();
            // The wrong lift keeps the given order; it is caught when
            // any pair the rewrite swapped arrives swapped.
            let pairs = [(x0, x1), (y0, y1), (z0, z1)];
            let mut swapped = false;
            for (i, &(lo, hi)) in pairs.iter().enumerate() {
                let (wlo, _whi) = wrong_norm(lo, hi);
                if wlo != seen[0].mins[i] {
                    swapped = true;
                }
            }
            if swapped {
                caught += 1;
            }
            drop(seen);
            cases += 1;
        }
        (cases, caught)
    }

    /// The swapped-operand expander: radius minus centre. Must be caught
    /// wherever subtraction is not symmetric.
    fn wrong_fsub(a: u32, b: u32) -> u32 {
        (f32::from_bits(b) - f32::from_bits(a)).to_bits()
    }

    /// Runs the extent test. Returns (comparisons, caught).
    fn run_expanded(seed: u32) -> (u32, u32) {
        let mut rng = Rng(seed);
        rt::set_callee(1, views_stub as *const () as u32);
        // A distinct nonzero routine cookie per seed.
        let cookie = 0x0E00_0000 | (seed & 0x00FF_FFFF);
        rt::set_relocated(TAG_VA, cookie);
        let routine = ExtentRoutine::new(cookie).expect("nonzero cookie");
        let proto = AreaProbe;
        let (mut cases, mut caught) = (0, 0);
        let words = expand_words(&mut rng);
        for n in 0..160 {
            let pick = |k: usize| words[(n * 5 + k * 3) % words.len()];
            let (x, y, z, r0, r1, r2) =
                (pick(0), pick(1), pick(2), pick(3), pick(4), pick(5));
            VIEWS_LOG.lock().unwrap().clear();
            let got = unsafe { fn_00B96B00::rw_00b96b00(x, y, z, r0, r1, r2) };
            assert_eq!(got, 0);
            assert_eq!(VIEWS_LOG.lock().unwrap().len(), 1);
            let mut lift_log = Vec::new();
            proto.test_expanded(
                &mut |row: [u32; 8], rtine: ExtentRoutine, view: [u32; 9], a: u32, b: u32| {
                    lift_log.push(ViewsCall {
                        row,
                        routine: rtine.get(),
                        view,
                        a,
                        b,
                    });
                },
                routine,
                x,
                y,
                z,
                r0,
                r1,
                r2,
            );
            assert_eq!(lift_log, *VIEWS_LOG.lock().unwrap());
            // The trailing words are the named constants on both sides.
            assert_eq!(VIEWS_LOG.lock().unwrap()[0].a, EXTENT_ARG_A);
            assert_eq!(VIEWS_LOG.lock().unwrap()[0].b, EXTENT_ARG_B);
            // The wrong lift swaps the expansion operands; it is caught
            // when any bound's subtraction is not symmetric.
            if wrong_fsub(x, r0) != (f32::from_bits(x) - f32::from_bits(r0)).to_bits()
                || wrong_fsub(y, r1) != (f32::from_bits(y) - f32::from_bits(r1)).to_bits()
                || wrong_fsub(z, r2) != (f32::from_bits(z) - f32::from_bits(r2)).to_bits()
            {
                caught += 1;
            }
            cases += 1;
        }
        (cases, caught)
    }

    #[test]
    fn point_matches() {
        let _guard = lock();
        let (cases, caught) = run_point(0xC001);
        assert!(cases > 100, "too few comparisons ({cases})");
        assert!(caught > 0, "inverted prime gate never caught ({cases} cases)");
    }

    #[test]
    fn box_matches() {
        let _guard = lock();
        let (cases, caught) = run_box(0xC002);
        assert!(cases > 100, "too few comparisons ({cases})");
        assert!(caught > 0, "unswapping normaliser never caught ({cases} cases)");
    }

    #[test]
    fn expanded_matches() {
        let _guard = lock();
        let (cases, caught) = run_expanded(0xC003);
        assert!(cases > 100, "too few comparisons ({cases})");
        assert!(caught > 0, "swapped expansion never caught ({cases} cases)");
    }
}
