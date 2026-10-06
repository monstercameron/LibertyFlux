//! Differential cases, part 3: the float predicate and the entry scanner.
//!
//! Each predicate case plants one index table, two threshold words, six
//! threshold constants, one probe address and two scripted answers, then
//! compares the byte answer with the lifted boolean and every call with
//! its arguments (the table word rebuilt per case). Each scanner case
//! plants a registry, entries with planted anchors, objects and slot
//! tables, then compares the full ordered call log with translated
//! addresses. Each method has a deliberately wrong lift that must be
//! caught. 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_input_frontend::input_ui::gates::{
        EntryKind, EntryNotify, EntryRefine, GateConsts, GateProbe, Membership, ScanConsts,
        ScanEntry, ScanGate, ScanQuery, ScanRegistry, float_gate,
    };
    use lf_inputui_diff::rewrites::*;
    use lf_inputui_diff::rt;
    use std::collections::VecDeque;
    use std::sync::Mutex;

    #[path = "../support/mod.rs"]
    mod support;
    use support::{Rng, addr, lock};

    /// Edge float bit patterns (mirrors the bounds binary's pool).
    fn edge_floats(rng: &mut Rng) -> Vec<u32> {
        // Single patterns with a comment each (kept clear of the
        // publication scan's long-hex-run rule).
        let mut v = std::vec![
            0x0000_0000, // +0.0
            0x8000_0000, // -0.0
            0x3F80_0000, // 1.0
            0xBF80_0000, // -1.0
            0x42C8_0000, // 100.0
            0xC2C8_0000, // -100.0
            0x7F7F_FFFF, // largest finite
            0xFF7F_FFFF, // most negative finite
            0x7F80_0000, // +inf
            0xFF80_0000, // -inf
            0x0000_0001, // smallest subnormal
            0x7FC0_0000, // canonical NaN
            0xFFC0_0000, // negative canonical NaN
            0x7FC0_1234, // quiet NaN with payload
        ];
        for _ in 0..6 {
            v.push(rng.u32());
        }
        v
    }

    // ---------------- the float predicate ----------------

    /// Rewrite-side predicate stub state.
    struct PredStubs {
        member_log: Vec<(u32, u32, u32, u32, u32, u32)>,
        member_answer: u32,
        probe_log: Vec<u32>,
        probe_answer: u32,
    }

    static PRED_STUBS: Mutex<PredStubs> = Mutex::new(PredStubs {
        member_log: Vec::new(),
        member_answer: 0,
        probe_log: Vec::new(),
        probe_answer: 0,
    });

    /// Rewrite-side membership stub (thiscall, six words).
    extern "thiscall" fn member_stub(
        this: u32,
        x2: u32,
        x1: u32,
        z0: u32,
        c5: u32,
        c0: u32,
    ) -> u32 {
        let mut stubs = PRED_STUBS.lock().unwrap();
        stubs.member_log.push((this, x2, x1, z0, c5, c0));
        stubs.member_answer
    }

    /// Rewrite-side probe stub (thiscall, one word).
    extern "thiscall" fn probe_stub(this: u32) -> u32 {
        let mut stubs = PRED_STUBS.lock().unwrap();
        stubs.probe_log.push(this);
        stubs.probe_answer
    }

    struct FakeMember {
        answer: bool,
        log: Vec<[f32; 3]>,
    }

    impl Membership for FakeMember {
        fn query(&mut self, point: [f32; 3]) -> bool {
            self.log.push(point);
            self.answer
        }
    }

    struct FakeProbe {
        answer: u32,
        calls: u32,
    }

    impl GateProbe for FakeProbe {
        fn probe(&mut self) -> u32 {
            self.calls += 1;
            self.answer
        }
    }

    /// Deliberately wrong predicate: the near-path floor arms are
    /// swapped (a flipped-branch slip). Must be caught whenever the
    /// near path reaches the floor check.
    #[allow(clippy::too_many_arguments)]
    fn wrong_gate(
        point: [f32; 3],
        anchor: [f32; 2],
        f2: f32,
        f3: f32,
        under: bool,
        consts: &GateConsts,
        present: bool,
        r2: u32,
    ) -> bool {
        let dx = anchor[0] - point[0];
        let dy = anchor[1] - point[1];
        let dist2 = dx * dx + dy * dy;
        if present {
            if !under {
                return false;
            }
        } else {
            if (r2 & 0xFF) == 0 {
                let k = consts.cap;
                let mut t = f3;
                if !(t > k) {
                    t = k;
                }
                t = t / k;
                t = t * consts.window;
                t = t * f2;
                t = t * t;
                if dist2 > t {
                    return false;
                }
                let s = consts.floor;
                let s2 = s * s;
                // WRONG: arms swapped.
                if !(s2 > dist2) {
                    return false;
                } else {
                    return true;
                }
            }
            if !under {
                return false;
            }
        }
        let k = consts.cap;
        let m = if k > f3 { k } else { f3 };
        let base = consts.base;
        let c3 = m * f2;
        let c3sq = c3 * c3;
        let c2 = (base - consts.inset) * f2;
        let c2sq = c2 * c2;
        let c1 = (base * consts.frac) * f2;
        let c1sq = c1 * c1;
        if dist2 > c3sq {
            return false;
        }
        if c2sq > dist2 {
            return false;
        }
        if c1sq > dist2 {
            return false;
        }
        true
    }

    #[test]
    fn float_gate_matches() {
        let _guard = lock();
        rt::set_callee(1, member_stub as usize as u32);
        rt::set_callee(2, probe_stub as usize as u32);
        let mut rng = Rng(0x9A7E);
        let mut cases = 0;
        let mut caught = 0;
        let edges = edge_floats(&mut rng);
        let probe_cell = Box::new(0xBEAD_0000u32);
        let probe_addr = addr(&*probe_cell);
        rt::set_relocated(support::PROBE_VA, probe_addr);
        // Scripted answers: absent/present crossed with near/far probe
        // bytes, with high-byte residue proving only the low byte decides.
        let scripts = [
            (0u32, 0x0000_0000u32),
            (0, 0x00AB_0000),
            (0, 0x0000_0001),
            (0, 0x00CD_0001),
            (1, 0x0000_0000),
            (0xFFFF_FFFF, 0x1234_5678),
        ];
        for &idx in &[0u32, 1, 2, 3] {
            for &(member_answer, r2) in &scripts {
                for &under_a in &[0u32, 1, 0xFFFF_FFFF] {
                    for &under_b in &[0u32, 1, 0xFFFF_FFFF] {
                        // A few float shapes per script combo.
                        for round in 0..6 {
                            let pick = |rng: &mut Rng| {
                                f32::from_bits(if rng.u32() % 2 == 0 {
                                    edges[(rng.u32() as usize) % edges.len()]
                                } else {
                                    rng.u32()
                                })
                            };
                            let point = [pick(&mut rng), pick(&mut rng), pick(&mut rng)];
                            let anchor = [pick(&mut rng), pick(&mut rng)];
                            let f2 = pick(&mut rng);
                            let f3 = pick(&mut rng);
                            let consts = GateConsts {
                                cap: pick(&mut rng),
                                window: pick(&mut rng),
                                floor: pick(&mut rng),
                                base: pick(&mut rng),
                                inset: pick(&mut rng),
                                frac: pick(&mut rng),
                            };
                            // Plant the table: word 0 is the index, the
                            // rest are garbage table words.
                            let mut tab = [rng.u32(), rng.u32(), rng.u32(), rng.u32()];
                            tab[0] = idx;
                            for (k, w) in tab.iter().enumerate() {
                                unsafe {
                                    rt::global::<u32>(
                                        support::FG_TAB_VA + (k as u32) * 4,
                                    )
                                    .write_unaligned(*w);
                                }
                            }
                            unsafe {
                                rt::global::<u32>(support::FG_UNDER_A_VA).write_unaligned(under_a);
                                rt::global::<u32>(support::FG_UNDER_B_VA).write_unaligned(under_b);
                                let words = [
                                    consts.cap.to_bits(),
                                    consts.window.to_bits(),
                                    consts.floor.to_bits(),
                                    consts.base.to_bits(),
                                    consts.inset.to_bits(),
                                    consts.frac.to_bits(),
                                ];
                                for (k, va) in support::FG_CONST_VAS.iter().enumerate() {
                                    rt::global::<u32>(*va).write_unaligned(words[k]);
                                }
                            }
                            {
                                let mut stubs = PRED_STUBS.lock().unwrap();
                                stubs.member_log.clear();
                                stubs.member_answer = member_answer;
                                stubs.probe_log.clear();
                                stubs.probe_answer = r2;
                            }
                            let point_box = Box::new(point);
                            let anchor_box = Box::new(anchor);
                            // Rewrite side.
                            let got = unsafe {
                                fn_00AFE3B0::rw_00afe3b0(
                                    addr(&point_box[0]),
                                    addr(&anchor_box[0]),
                                    f2,
                                    f3,
                                )
                            };
                            let (member_log, probe_log);
                            {
                                let stubs = PRED_STUBS.lock().unwrap();
                                member_log = stubs.member_log.clone();
                                probe_log = stubs.probe_log.clone();
                            }
                            // Lift side.
                            let mut member = FakeMember {
                                answer: member_answer != 0,
                                log: Vec::new(),
                            };
                            let mut probe = FakeProbe {
                                answer: r2,
                                calls: 0,
                            };
                            let lift = float_gate(
                                point,
                                anchor,
                                f2,
                                f3,
                                under_a < under_b,
                                &consts,
                                &mut member,
                                &mut probe,
                            );
                            // Compare.
                            assert_eq!(got, u8::from(lift), "round={round}");
                            assert!(got == 0 || got == 1);
                            assert_eq!(member_log.len(), 1);
                            let (this, x2, x1, z0, c5, c0) = member_log[0];
                            assert_eq!(this, tab[idx as usize].wrapping_add(0x10));
                            assert_eq!((x2, x1, z0), (point[0].to_bits(), point[1].to_bits(), point[2].to_bits()));
                            assert_eq!((c5, c0), (0x40A0_0000, 0));
                            assert_eq!(member.log, std::vec![point]);
                            if member_answer == 0 {
                                assert_eq!(probe_log, std::vec![probe_addr]);
                                assert_eq!(probe.calls, 1);
                            } else {
                                assert!(probe_log.is_empty());
                                assert_eq!(probe.calls, 0);
                            }
                            if wrong_gate(
                                point,
                                anchor,
                                f2,
                                f3,
                                under_a < under_b,
                                &consts,
                                member_answer != 0,
                                r2,
                            ) != lift
                            {
                                caught += 1;
                            }
                            cases += 1;
                            std::hint::black_box((&point_box, &anchor_box));
                        }
                    }
                }
            }
        }
        std::hint::black_box(&probe_cell);
        assert!(cases > 200, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong gate never caught ({cases} cases)");
    }

