//! Differential cases, part 2: the altitude gate against its eight
//! instances in five tail shapes.
//!
//! Each case plants the threshold word, runs the rewrite and the matching
//! [`AltitudeGate`] method on the same words, and compares the ground
//! query (presence and arguments), the converted triple snapshotted
//! inside the tail stub, every trailing word, and the call order. The
//! ground answers are scripted identically on both sides. A deliberately
//! wrong lift (the gate inverted: converting above the threshold instead
//! of below) must be caught per tail shape. 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use std::collections::VecDeque;
    use std::sync::Mutex;

    use lf_scriptvmdiff::rewrites::*;
    use lf_scriptvmdiff::rt;
    use lf_script::script_vm::{AltitudeGate, CONV_MODE};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{Rng, THRESH_VA, lock, snap};

    /// One recorded ground-query call: x, y, mode.
    static CONV_LOG: Mutex<Vec<(u32, u32, u32)>> = Mutex::new(Vec::new());
    /// Scripted ground answers, in bits, popped per call.
    static CONV_SCRIPT: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());

    extern "cdecl" fn conv_stub(x: u32, y: u32, mode: u32) -> f32 {
        CONV_LOG.lock().unwrap().push((x, y, mode));
        f32::from_bits(CONV_SCRIPT.lock().unwrap().pop_front().unwrap_or(0))
    }

    /// One recorded tail call: converted triple plus trailing words.
    #[derive(Debug, PartialEq, Eq)]
    struct TailCall {
        point: [u32; 3],
        rest: Vec<u32>,
    }

    static TAIL_LOG: Mutex<Vec<TailCall>> = Mutex::new(Vec::new());

    /// Snapshot helper shared by the tail stubs.
    fn record_tail(buf: u32, rest: &[u32]) {
        let words = unsafe { snap(buf, 3) };
        TAIL_LOG.lock().unwrap().push(TailCall {
            point: [words[0], words[1], words[2]],
            rest: rest.to_vec(),
        });
    }

    extern "cdecl" fn reg_stub(buf: u32, w: u32, extra: u32) -> u32 {
        record_tail(buf, &[w, extra]);
        0
    }

    extern "cdecl" fn clear6_stub(buf: u32, w: u32, extra: u32, z0: u32, z1: u32, z2: u32) -> u32 {
        record_tail(buf, &[w, extra, z0, z1, z2]);
        0
    }

    extern "cdecl" fn tail4_stub(buf: u32, w: u32, a: u32, b: u32, c: u32) -> u32 {
        record_tail(buf, &[w, a, b, c]);
        0
    }

    extern "cdecl" fn reg2_stub(buf: u32, w: u32) -> u32 {
        record_tail(buf, &[w]);
        0
    }

    /// Thresholds: the game's -100.0, zero, infinities, NaN, randoms.
    fn thresholds(rng: &mut Rng) -> Vec<u32> {
        // -100.0 as bits, then signed zeros and infinities.
        let mut out = vec![
            0xC2C8_0000,
            0x0000_0000,
            0x8000_0000,
            0x7F80_0000,
            0xFF80_0000,
        ];
        // Quiet and signalling NaN bits.
        out.push(0x7FC0_0001);
        out.push(0x7F80_0001);
        for _ in 0..4 {
            out.push(rng.u32());
        }
        out
    }

    /// Altitudes around a threshold: either side, the value itself,
    /// zeros, infinities, NaNs, subnormals, randoms.
    fn altitudes(rng: &mut Rng, thresh: u32) -> Vec<u32> {
        // Neighbouring bit patterns just above and below the threshold.
        let mut out = vec![
            thresh,
            thresh.wrapping_add(1),
            thresh.wrapping_sub(1),
            0x0000_0000,
            0x8000_0000,
            0x7F80_0000,
            0xFF80_0000,
            0x7FC0_0000,
            0xFFC0_0000,
            0x0000_0001,
        ];
        for _ in 0..6 {
            out.push(rng.u32());
        }
        out
    }

    /// Plants the threshold global both sides read.
    fn plant_thresh(thresh: u32) {
        unsafe { rt::global::<u32>(THRESH_VA).write(thresh) };
    }

    /// Whether the 32-bit head converts: the threshold at or above `z`,
    /// ordered (NaN on either side keeps `z`).
    fn converts(thresh: u32, z: u32) -> bool {
        f32::from_bits(thresh) >= f32::from_bits(z)
    }

    /// Quiets a signalling-NaN answer the way the x87 return path does.
    fn quiet(bits: u32) -> u32 {
        if bits & 0x7F80_0000 == 0x7F80_0000 && bits & 0x007F_FFFF != 0 && bits & 0x0040_0000 == 0
        {
            bits | 0x0040_0000
        } else {
            bits
        }
    }

    /// Signalling-NaN answers, scripted in rotation to pin the quieting.
    const SNAN_ANSWERS: [u32; 3] = [
        0x7F80_0001, // positive, smallest payload
        0xFF80_7FFF, // negative, large payload
        0x7F80_1234, // positive, mid payload
    ];

    /// The inverted gate: converts above instead of below. Must be caught
    /// wherever the scripted answer differs from `z`.
    fn wrong_conv(thresh: u32, z: u32, answer: u32) -> u32 {
        if f32::from_bits(thresh) >= f32::from_bits(z) {
            z
        } else {
            answer
        }
    }

    type RestartFn = extern "cdecl" fn(u32, u32, u32, u32, u32) -> u32;
    type QuadFn = extern "cdecl" fn(u32, u32, u32, u32) -> u32;

    /// Runs one restart-tail instance (5 args, tail `register`).
    fn run_restart(skip: RestartFn, seed: u32) -> (u32, u32) {
        let mut rng = Rng(seed);
        rt::set_callee(1, conv_stub as *const () as u32);
        rt::set_callee(2, reg_stub as *const () as u32);
        let (mut cases, mut caught) = (0, 0);
        for thresh in thresholds(&mut rng) {
            let gate = AltitudeGate::new(thresh);
            for z in altitudes(&mut rng, thresh) {
                let (x, y, w, extra) = (rng.u32(), rng.u32(), rng.u32(), rng.u32());
                // Every ninth case scripts a signalling NaN so the
                // x87 quieting is pinned, not merely hoped for.
                let answer = if cases % 9 == 8 {
                    SNAN_ANSWERS[(cases / 9) as usize % SNAN_ANSWERS.len()]
                } else {
                    rng.u32()
                };
                plant_thresh(thresh);
                CONV_LOG.lock().unwrap().clear();
                TAIL_LOG.lock().unwrap().clear();
                CONV_SCRIPT.lock().unwrap().clear();
                CONV_SCRIPT.lock().unwrap().push_back(answer);
                let got = unsafe { skip(x, y, z, w, extra) };
                assert_eq!(got, 0);
                let conv = converts(thresh, z);
                assert_eq!(
                    *CONV_LOG.lock().unwrap(),
                    if conv { vec![(x, y, CONV_MODE)] } else { vec![] },
                    "thresh={thresh:#x} z={z:#x}"
                );
                let want_point = [x, y, if conv { quiet(answer) } else { z }];
                assert_eq!(
                    *TAIL_LOG.lock().unwrap(),
                    [TailCall {
                        point: want_point,
                        rest: vec![w, extra],
                    }],
                    "thresh={thresh:#x} z={z:#x}"
                );
                // The lift must make the same calls in the same order.
                let mut lift_conv = Vec::new();
                let mut lift_tail = Vec::new();
                gate.register_restart(
                    &mut |qx: u32, qy: u32, mode: u32| {
                        lift_conv.push((qx, qy, mode));
                        answer
                    },
                    &mut |point: [u32; 3], lw: u32, le: u32| {
                        lift_tail.push(TailCall {
                            point,
                            rest: vec![lw, le],
                        });
                    },
                    x,
                    y,
                    z,
                    w,
                    extra,
                );
                assert_eq!(lift_conv, *CONV_LOG.lock().unwrap());
                assert_eq!(lift_tail, *TAIL_LOG.lock().unwrap());
                if wrong_conv(thresh, z, answer) != want_point[2] {
                    caught += 1;
                }
                cases += 1;
            }
        }
        (cases, caught)
    }

    /// Runs the full-area tail instance (5 args, tail `clear` + zeros).
    fn run_clear6(seed: u32) -> (u32, u32) {
        let mut rng = Rng(seed);
        rt::set_callee(1, conv_stub as *const () as u32);
        rt::set_callee(2, clear6_stub as *const () as u32);
        let (mut cases, mut caught) = (0, 0);
        for thresh in thresholds(&mut rng) {
            let gate = AltitudeGate::new(thresh);
            for z in altitudes(&mut rng, thresh) {
                let (x, y, w, extra) = (rng.u32(), rng.u32(), rng.u32(), rng.u32());
                // Every ninth case scripts a signalling NaN so the
                // x87 quieting is pinned, not merely hoped for.
                let answer = if cases % 9 == 8 {
                    SNAN_ANSWERS[(cases / 9) as usize % SNAN_ANSWERS.len()]
                } else {
                    rng.u32()
                };
                plant_thresh(thresh);
                CONV_LOG.lock().unwrap().clear();
                TAIL_LOG.lock().unwrap().clear();
                CONV_SCRIPT.lock().unwrap().clear();
                CONV_SCRIPT.lock().unwrap().push_back(answer);
                let got = unsafe { fn_00B958C0::rw_00b958c0(x, y, z, w, extra) };
                assert_eq!(got, 0);
                let conv = converts(thresh, z);
                assert_eq!(
                    *CONV_LOG.lock().unwrap(),
                    if conv { vec![(x, y, CONV_MODE)] } else { vec![] },
                    "thresh={thresh:#x} z={z:#x}"
                );
                let want_point = [x, y, if conv { quiet(answer) } else { z }];
                assert_eq!(
                    *TAIL_LOG.lock().unwrap(),
                    [TailCall {
                        point: want_point,
                        rest: vec![w, extra, 0, 0, 0],
                    }],
                    "thresh={thresh:#x} z={z:#x}"
                );
                let mut lift_conv = Vec::new();
                let mut lift_tail = Vec::new();
                gate.clear_area(
                    &mut |qx: u32, qy: u32, mode: u32| {
                        lift_conv.push((qx, qy, mode));
                        answer
                    },
                    &mut |point: [u32; 3], lw: u32, le: u32, z0: u32, z1: u32, z2: u32| {
                        lift_tail.push(TailCall {
                            point,
                            rest: vec![lw, le, z0, z1, z2],
                        });
                    },
                    x,
                    y,
                    z,
                    w,
                    extra,
                );
                assert_eq!(lift_conv, *CONV_LOG.lock().unwrap());
                assert_eq!(lift_tail, *TAIL_LOG.lock().unwrap());
                if wrong_conv(thresh, z, answer) != want_point[2] {
                    caught += 1;
                }
                cases += 1;
            }
        }
        (cases, caught)
    }

    /// Runs the flag-tail instance (4 args, tail `emit` with 0, 1, 0).
    fn run_cars(seed: u32) -> (u32, u32) {
        let mut rng = Rng(seed);
        rt::set_callee(1, conv_stub as *const () as u32);
        rt::set_callee(2, tail4_stub as *const () as u32);
        let (mut cases, mut caught) = (0, 0);
        for thresh in thresholds(&mut rng) {
            let gate = AltitudeGate::new(thresh);
            for z in altitudes(&mut rng, thresh) {
                let (x, y, w) = (rng.u32(), rng.u32(), rng.u32());
                // Every ninth case scripts a signalling NaN so the
                // x87 quieting is pinned, not merely hoped for.
                let answer = if cases % 9 == 8 {
                    SNAN_ANSWERS[(cases / 9) as usize % SNAN_ANSWERS.len()]
                } else {
                    rng.u32()
                };
                plant_thresh(thresh);
                CONV_LOG.lock().unwrap().clear();
                TAIL_LOG.lock().unwrap().clear();
                CONV_SCRIPT.lock().unwrap().clear();
                CONV_SCRIPT.lock().unwrap().push_back(answer);
                let got = unsafe { fn_00B95950::rw_00b95950(x, y, z, w) };
                assert_eq!(got, 0);
                let conv = converts(thresh, z);
                assert_eq!(
                    *CONV_LOG.lock().unwrap(),
                    if conv { vec![(x, y, CONV_MODE)] } else { vec![] },
                    "thresh={thresh:#x} z={z:#x}"
                );
                let want_point = [x, y, if conv { quiet(answer) } else { z }];
                assert_eq!(
                    *TAIL_LOG.lock().unwrap(),
                    [TailCall {
                        point: want_point,
                        rest: vec![w, 0, 1, 0],
                    }],
                    "thresh={thresh:#x} z={z:#x}"
                );
                let mut lift_conv = Vec::new();
                let mut lift_tail = Vec::new();
                gate.clear_area_cars(
                    &mut |qx: u32, qy: u32, mode: u32| {
                        lift_conv.push((qx, qy, mode));
                        answer
                    },
                    &mut |point: [u32; 3], lw: u32, a: u32, b: u32, c: u32| {
                        lift_tail.push(TailCall {
                            point,
                            rest: vec![lw, a, b, c],
                        });
                    },
                    x,
                    y,
                    z,
                    w,
                );
                assert_eq!(lift_conv, *CONV_LOG.lock().unwrap());
                assert_eq!(lift_tail, *TAIL_LOG.lock().unwrap());
                if wrong_conv(thresh, z, answer) != want_point[2] {
                    caught += 1;
                }
                cases += 1;
            }
        }
        (cases, caught)
    }

    /// Runs one single-register instance (4 args, tail `register`).
    fn run_reg2(skip: QuadFn, seed: u32) -> (u32, u32) {
        let mut rng = Rng(seed);
        rt::set_callee(1, conv_stub as *const () as u32);
        rt::set_callee(2, reg2_stub as *const () as u32);
        let (mut cases, mut caught) = (0, 0);
        for thresh in thresholds(&mut rng) {
            let gate = AltitudeGate::new(thresh);
            for z in altitudes(&mut rng, thresh) {
                let (x, y, w) = (rng.u32(), rng.u32(), rng.u32());
                // Every ninth case scripts a signalling NaN so the
                // x87 quieting is pinned, not merely hoped for.
                let answer = if cases % 9 == 8 {
                    SNAN_ANSWERS[(cases / 9) as usize % SNAN_ANSWERS.len()]
                } else {
                    rng.u32()
                };
                plant_thresh(thresh);
                CONV_LOG.lock().unwrap().clear();
                TAIL_LOG.lock().unwrap().clear();
                CONV_SCRIPT.lock().unwrap().clear();
                CONV_SCRIPT.lock().unwrap().push_back(answer);
                let got = unsafe { skip(x, y, z, w) };
                assert_eq!(got, 0);
                let conv = converts(thresh, z);
                assert_eq!(
                    *CONV_LOG.lock().unwrap(),
                    if conv { vec![(x, y, CONV_MODE)] } else { vec![] },
                    "thresh={thresh:#x} z={z:#x}"
                );
                let want_point = [x, y, if conv { quiet(answer) } else { z }];
                assert_eq!(
                    *TAIL_LOG.lock().unwrap(),
                    [TailCall {
                        point: want_point,
                        rest: vec![w],
                    }],
                    "thresh={thresh:#x} z={z:#x}"
                );
                let mut lift_conv = Vec::new();
                let mut lift_tail = Vec::new();
                gate.register_point(
                    &mut |qx: u32, qy: u32, mode: u32| {
                        lift_conv.push((qx, qy, mode));
                        answer
                    },
                    &mut |point: [u32; 3], lw: u32| {
                        lift_tail.push(TailCall {
                            point,
                            rest: vec![lw],
                        });
                    },
                    x,
                    y,
                    z,
                    w,
                );
                assert_eq!(lift_conv, *CONV_LOG.lock().unwrap());
                assert_eq!(lift_tail, *TAIL_LOG.lock().unwrap());
                if wrong_conv(thresh, z, answer) != want_point[2] {
                    caught += 1;
                }
                cases += 1;
            }
        }
        (cases, caught)
    }

    /// Runs the double-tail instance (4 args: `register`, then zeros).
    /// The two stubs share one log, so the call order is compared too.
    fn run_objects(seed: u32) -> (u32, u32) {
        let mut rng = Rng(seed);
        rt::set_callee(1, conv_stub as *const () as u32);
        rt::set_callee(2, reg2_stub as *const () as u32);
        rt::set_callee(3, tail4_stub as *const () as u32);
        let (mut cases, mut caught) = (0, 0);
        for thresh in thresholds(&mut rng) {
            let gate = AltitudeGate::new(thresh);
            for z in altitudes(&mut rng, thresh) {
                let (x, y, w) = (rng.u32(), rng.u32(), rng.u32());
                // Every ninth case scripts a signalling NaN so the
                // x87 quieting is pinned, not merely hoped for.
                let answer = if cases % 9 == 8 {
                    SNAN_ANSWERS[(cases / 9) as usize % SNAN_ANSWERS.len()]
                } else {
                    rng.u32()
                };
                plant_thresh(thresh);
                CONV_LOG.lock().unwrap().clear();
                TAIL_LOG.lock().unwrap().clear();
                CONV_SCRIPT.lock().unwrap().clear();
                CONV_SCRIPT.lock().unwrap().push_back(answer);
                let got = unsafe { fn_00B95AE0::rw_00b95ae0(x, y, z, w) };
                assert_eq!(got, 0);
                let conv = converts(thresh, z);
                assert_eq!(
                    *CONV_LOG.lock().unwrap(),
                    if conv { vec![(x, y, CONV_MODE)] } else { vec![] },
                    "thresh={thresh:#x} z={z:#x}"
                );
                let want_point = [x, y, if conv { quiet(answer) } else { z }];
                assert_eq!(
                    *TAIL_LOG.lock().unwrap(),
                    [
                        TailCall {
                            point: want_point,
                            rest: vec![w],
                        },
                        TailCall {
                            point: want_point,
                            rest: vec![w, 0, 0, 0],
                        },
                    ],
                    "thresh={thresh:#x} z={z:#x}"
                );
                let mut lift_conv = Vec::new();
                // The two tails log separately (one closure cannot lend
                // the same log twice); order is pinned by concatenation.
                let mut lift_first = Vec::new();
                let mut lift_second = Vec::new();
                gate.clear_objects(
                    &mut |qx: u32, qy: u32, mode: u32| {
                        lift_conv.push((qx, qy, mode));
                        answer
                    },
                    &mut |point: [u32; 3], lw: u32| {
                        lift_first.push(TailCall {
                            point,
                            rest: vec![lw],
                        });
                    },
                    &mut |point: [u32; 3], lw: u32, a: u32, b: u32, c: u32| {
                        lift_second.push(TailCall {
                            point,
                            rest: vec![lw, a, b, c],
                        });
                    },
                    x,
                    y,
                    z,
                    w,
                );
                assert_eq!(lift_conv, *CONV_LOG.lock().unwrap());
                lift_first.extend(lift_second);
                assert_eq!(lift_first, *TAIL_LOG.lock().unwrap());
                if wrong_conv(thresh, z, answer) != want_point[2] {
                    caught += 1;
                }
                cases += 1;
            }
        }
        (cases, caught)
    }

    macro_rules! alt_test {
        ($name:ident, $run:expr, $seed:expr) => {
            #[test]
            fn $name() {
                let _guard = lock();
                let (cases, caught) = $run;
                assert!(cases > 100, "too few comparisons ({cases})");
                assert!(caught > 0, "inverted gate never caught ({cases} cases)");
            }
        };
    }

    alt_test!(
        restart_a_matches,
        run_restart(fn_00B95520::rw_00b95520, 0xA101),
        0xA101
    );
    alt_test!(
        restart_b_matches,
        run_restart(fn_00B955A0::rw_00b955a0, 0xA102),
        0xA102
    );
    alt_test!(clear_c_matches, run_clear6(0xA103), 0xA103);
    alt_test!(cars_d_matches, run_cars(0xA104), 0xA104);
    alt_test!(
        reg_e_matches,
        run_reg2(fn_00B959E0::rw_00b959e0, 0xA105),
        0xA105
    );
    alt_test!(
        reg_f_matches,
        run_reg2(fn_00B95A60::rw_00b95a60, 0xA106),
        0xA106
    );
    alt_test!(objects_g_matches, run_objects(0xA107), 0xA107);
    alt_test!(
        reg_h_matches,
        run_reg2(fn_00B96DE0::rw_00b96de0, 0xA108),
        0xA108
    );
}
