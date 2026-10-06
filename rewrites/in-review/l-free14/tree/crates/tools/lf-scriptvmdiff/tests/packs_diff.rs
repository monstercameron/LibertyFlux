//! Differential cases, part 5: the two pack-and-forward singles.
//!
//! Each case runs the rewrite and the matching lifted method on the same
//! words and compares every forwarded window or triple (snapshotted
//! inside the stub) with every scalar. Deliberately wrong lifts (colour
//! channels swapped; triples swapped) must be caught. 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use std::sync::Mutex;

    use lf_scriptvmdiff::rewrites::*;
    use lf_scriptvmdiff::rt;
    use lf_script::script_vm::{AngledArea, LoadingClock};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{Rng, lock, snap};

    /// One recorded clock draw: five windows plus four scalars.
    #[derive(Debug, PartialEq, Eq)]
    struct DrawCall {
        win2: [u32; 2],
        win4: [u32; 4],
        win6: [u32; 6],
        win8: [u32; 8],
        win10: [u32; 10],
        int: u32,
        color: u32,
        tail0: u32,
        tail1: u32,
    }

    static DRAW_LOG: Mutex<Vec<DrawCall>> = Mutex::new(Vec::new());

    extern "cdecl" fn draw_stub(
        w2: u32,
        w4: u32,
        w6: u32,
        w8: u32,
        w10: u32,
        int: u32,
        color: u32,
        tail0: u32,
        tail1: u32,
    ) -> u32 {
        DRAW_LOG.lock().unwrap().push(DrawCall {
            win2: unsafe { snap(w2, 2) }.try_into().unwrap(),
            win4: unsafe { snap(w4, 4) }.try_into().unwrap(),
            win6: unsafe { snap(w6, 6) }.try_into().unwrap(),
            win8: unsafe { snap(w8, 8) }.try_into().unwrap(),
            win10: unsafe { snap(w10, 10) }.try_into().unwrap(),
            int,
            color,
            tail0,
            tail1,
        });
        0
    }

    /// One recorded angled clear: two triples, flag, trailing zero.
    #[derive(Debug, PartialEq, Eq)]
    struct ClearCall {
        near: [u32; 3],
        far: [u32; 3],
        flag: u32,
        zero: u32,
    }

    static CLEAR_LOG: Mutex<Vec<ClearCall>> = Mutex::new(Vec::new());

    extern "cdecl" fn clear_stub(near: u32, far: u32, flag: u32, zero: u32) -> u32 {
        let n = unsafe { snap(near, 3) };
        let f = unsafe { snap(far, 3) };
        CLEAR_LOG.lock().unwrap().push(ClearCall {
            near: [n[0], n[1], n[2]],
            far: [f[0], f[1], f[2]],
            flag,
            zero,
        });
        0
    }

    /// Packs the colour the way the rewrite does.
    fn pack_color(c11: u32, c12: u32, c13: u32, c14: u32) -> u32 {
        ((c14 & 0xFF) << 24) | ((c11 & 0xFF) << 16) | ((c12 & 0xFF) << 8) | (c13 & 0xFF)
    }

    /// The wrong colour: the middle channels swapped. Must be caught
    /// wherever the low bytes of `c11` and `c12` differ.
    fn wrong_color(c11: u32, c12: u32, c13: u32, c14: u32) -> u32 {
        ((c14 & 0xFF) << 24) | ((c12 & 0xFF) << 16) | ((c11 & 0xFF) << 8) | (c13 & 0xFF)
    }

    /// Runs the loading-clock routine. Returns (comparisons, caught).
    fn run_clock(seed: u32) -> (u32, u32) {
        let mut rng = Rng(seed);
        rt::set_callee(1, draw_stub as *const () as u32);
        let proto = LoadingClock;
        let (mut cases, mut caught) = (0, 0);
        for _ in 0..120 {
            // Seventeen script words: ten frame words, an integer, four
            // colour words and two trailing integers.
            let w: Vec<u32> = (0..17).map(|_| rng.u32()).collect();
            let (f0, f1, f2, f3, f4, a5, f6, f7, f8, f9) =
                (w[0], w[1], w[2], w[3], w[4], w[5], w[6], w[7], w[8], w[9]);
            let (i10, c11, c12, c13, c14, i15, i16) =
                (w[10], w[11], w[12], w[13], w[14], w[15], w[16]);
            DRAW_LOG.lock().unwrap().clear();
            let got = unsafe {
                fn_00B91170::rw_00b91170(
                    f0, f1, f2, f3, f4, a5, f6, f7, f8, f9, i10, c11, c12, c13, c14, i15, i16,
                )
            };
            assert_eq!(got, 0);
            // The frame order is pinned against the rewrite's snapshots,
            // not assumed from the lift.
            let frame = [f8, f9, f6, f7, f4, a5, f2, f3, f0, f1];
            let want = DrawCall {
                win2: [frame[8], frame[9]],
                win4: [frame[6], frame[7], frame[8], frame[9]],
                win6: [
                    frame[4], frame[5], frame[6], frame[7], frame[8], frame[9],
                ],
                win8: [
                    frame[2],
                    frame[3],
                    frame[4],
                    frame[5],
                    frame[6],
                    frame[7],
                    frame[8],
                    frame[9],
                ],
                win10: frame,
                int: i10,
                color: pack_color(c11, c12, c13, c14),
                tail0: i15,
                tail1: i16,
            };
            assert_eq!(*DRAW_LOG.lock().unwrap(), [want]);
            let mut lift_log = Vec::new();
            proto.draw(
                &mut |win2: [u32; 2],
                      win4: [u32; 4],
                      win6: [u32; 6],
                      win8: [u32; 8],
                      win10: [u32; 10],
                      int: u32,
                      color: u32,
                      tail0: u32,
                      tail1: u32| {
                    lift_log.push(DrawCall {
                        win2,
                        win4,
                        win6,
                        win8,
                        win10,
                        int,
                        color,
                        tail0,
                        tail1,
                    });
                },
                f0,
                f1,
                f2,
                f3,
                f4,
                a5,
                f6,
                f7,
                f8,
                f9,
                i10,
                c11,
                c12,
                c13,
                c14,
                i15,
                i16,
            );
            assert_eq!(lift_log, *DRAW_LOG.lock().unwrap());
            if wrong_color(c11, c12, c13, c14) != pack_color(c11, c12, c13, c14) {
                caught += 1;
            }
            cases += 1;
        }
        (cases, caught)
    }

    /// Runs the angled-area routine. Returns (comparisons, caught).
    fn run_angled(seed: u32) -> (u32, u32) {
        let mut rng = Rng(seed);
        rt::set_callee(1, clear_stub as *const () as u32);
        let proto = AngledArea;
        let (mut cases, mut caught) = (0, 0);
        for _ in 0..120 {
            let w: Vec<u32> = (0..7).map(|_| rng.u32()).collect();
            CLEAR_LOG.lock().unwrap().clear();
            let got = unsafe { fn_00B95850::rw_00b95850(w[0], w[1], w[2], w[3], w[4], w[5], w[6]) };
            assert_eq!(got, 0);
            let want = ClearCall {
                near: [w[0], w[1], w[2]],
                far: [w[3], w[4], w[5]],
                flag: w[6],
                zero: 0,
            };
            assert_eq!(*CLEAR_LOG.lock().unwrap(), [want]);
            let mut lift_log = Vec::new();
            proto.clear(
                &mut |near: [u32; 3], far: [u32; 3], flag: u32, zero: u32| {
                    lift_log.push(ClearCall {
                        near,
                        far,
                        flag,
                        zero,
                    });
                },
                w[0],
                w[1],
                w[2],
                w[3],
                w[4],
                w[5],
                w[6],
            );
            assert_eq!(lift_log, *CLEAR_LOG.lock().unwrap());
            // Wrong lift: the triples swapped. Caught wherever they differ.
            if [w[0], w[1], w[2]] != [w[3], w[4], w[5]] {
                caught += 1;
            }
            cases += 1;
        }
        (cases, caught)
    }

    #[test]
    fn clock_matches() {
        let _guard = lock();
        let (cases, caught) = run_clock(0xE101);
        assert!(cases > 100, "too few comparisons ({cases})");
        assert!(caught > 0, "swapped channels never caught ({cases} cases)");
    }

    #[test]
    fn angled_matches() {
        let _guard = lock();
        let (cases, caught) = run_angled(0xE102);
        assert!(cases > 100, "too few comparisons ({cases})");
        assert!(caught > 0, "swapped triples never caught ({cases} cases)");
    }
}
