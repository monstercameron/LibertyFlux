//! Differential case: the flee periodic update slot against its verified
//! rewrite on the same generated inputs.
//!
//! Compares the return value, the full task blob, and the probe, check
//! and dispatch calls against the lift's trait calls, both sides given
//! the same scripted answers. The check stub lives in a fake task
//! table. A deliberately wrong lift must be caught at least once.
//! 32-bit only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use core::sync::atomic::{AtomicU32, Ordering};

    use lf_core::Handle32;
    use lf_peds_tasks::tasks::{FleePed, FleePoll, FleeTask};
    use lf_taskdiff::rewrites::*;
    use lf_taskdiff::{set_callee, set_threshold};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        FL_FLAG_BYTE, FL_KIND, FL_MARKS, FL_MODE, FL_POS, FL_STATE_BYTE, FL_SUB, FL_VTABLE, Rng,
        U32_EDGE, addr, blob_byte, fake_table, flee_blob, ped_blob, set_blob_byte,
    };

    // Probe stub (slot 1) and its recording.
    static PROBE_TASK: AtomicU32 = AtomicU32::new(0);
    static PROBE_PED: AtomicU32 = AtomicU32::new(0);
    static PROBE_ANS: AtomicU32 = AtomicU32::new(0);
    static PROBE_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn probe_stub(task: u32, ped: u32) -> u32 {
        PROBE_TASK.store(task, Ordering::SeqCst);
        PROBE_PED.store(ped, Ordering::SeqCst);
        PROBE_COUNT.fetch_add(1, Ordering::SeqCst);
        PROBE_ANS.load(Ordering::SeqCst)
    }

    // Check stub (task slot +0x14) and its recording.
    static CHECK_TASK: AtomicU32 = AtomicU32::new(0);
    static CHECK_PED: AtomicU32 = AtomicU32::new(0);
    static CHECK_ONE: AtomicU32 = AtomicU32::new(0);
    static CHECK_ZERO: AtomicU32 = AtomicU32::new(0);
    static CHECK_ANS: AtomicU32 = AtomicU32::new(0);
    static CHECK_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn check_stub(task: u32, ped: u32, one: u32, zero: u32) -> u32 {
        CHECK_TASK.store(task, Ordering::SeqCst);
        CHECK_PED.store(ped, Ordering::SeqCst);
        CHECK_ONE.store(one, Ordering::SeqCst);
        CHECK_ZERO.store(zero, Ordering::SeqCst);
        CHECK_COUNT.fetch_add(1, Ordering::SeqCst);
        CHECK_ANS.load(Ordering::SeqCst)
    }

    // Dispatch stub (slot 3) and its recording.
    static DISP_TASK: AtomicU32 = AtomicU32::new(0);
    static DISP_PED: AtomicU32 = AtomicU32::new(0);
    static DISP_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn dispatch_stub(task: u32, ped: u32) -> u32 {
        DISP_TASK.store(task, Ordering::SeqCst);
        DISP_PED.store(ped, Ordering::SeqCst);
        DISP_COUNT.fetch_add(1, Ordering::SeqCst);
        0
    }

    fn probe_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32) -> u32 = probe_stub;
        f as usize as u32
    }

    fn check_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 = check_stub;
        f as usize as u32
    }

    fn dispatch_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32) -> u32 = dispatch_stub;
        f as usize as u32
    }

    struct Fake {
        probe_answer: u32,
        check_answer: u32,
        probes: Vec<u32>,
        checks: Vec<u32>,
        dispatches: Vec<u32>,
    }

    impl FleePoll for Fake {
        fn probe(&mut self, ped: Option<Handle32<FleePed>>) -> u32 {
            self.probes.push(Handle32::raw_or_zero(ped));
            self.probe_answer
        }

        fn check(&mut self, ped: Option<Handle32<FleePed>>) -> u32 {
            self.checks.push(Handle32::raw_or_zero(ped));
            self.check_answer
        }

        fn dispatch(&mut self, ped: Option<Handle32<FleePed>>) {
            self.dispatches.push(Handle32::raw_or_zero(ped));
        }
    }

    /// Pinned-order helpers for the inverted gate below.
    fn mul(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) * core::hint::black_box(b)
    }

    fn add(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) + core::hint::black_box(b)
    }

    /// Probes exactly where the gate rests (the inverted liveness test).
    fn wrong_poll(
        task: &mut FleeTask,
        ped: Option<Handle32<FleePed>>,
        threshold: f32,
        poll: &mut Fake,
    ) -> Option<Handle32<lf_peds_tasks::tasks::SubTask>> {
        let pos = task.pos();
        let live = if task.flag() {
            task.mode().is_some()
        } else if task.mode().is_some() {
            false
        } else {
            let sq = add(
                add(mul(pos[0], pos[0]), mul(pos[1], pos[1])),
                mul(pos[2], pos[2]),
            );
            sq > threshold
        };
        if !live && poll.probe(ped) & 0xFF != 0 {
            return task.subtask();
        }
        if task.marks() & 1 == 0 {
            if poll.check(ped) & 0xFF == 0 {
                return task.subtask();
            }
            *task = FleeTask::new(
                task.subtask(),
                task.marks() | 2,
                task.kind(),
                task.pos(),
                task.flag(),
                task.mode(),
                task.state(),
            );
        }
        poll.dispatch(ped);
        None
    }

    #[test]
    fn flee_poll_matches() {
        // Check slot byte offset 0x14, as a word index.
        const CHECK_SLOT: usize = 0x14 / 4;
        set_callee(1, probe_addr());
        // Slot 2 is unused by this method; slot 3 is the dispatch.
        set_callee(3, dispatch_addr());
        let mut rng = Rng(0xF1E0);
        let mut caught = 0;
        let mut cases = 0;
        let bare = 0.0f32.to_bits();
        let one = 1.0f32.to_bits();
        let three = 3.0f32.to_bits();
        let four = 4.0f32.to_bits();
        let nan = f32::NAN.to_bits();
        let inf = f32::INFINITY.to_bits();
        let coords = [bare, one, three, four, nan, inf, rng.u32()];
        let small = 0.05f32.to_bits();
        let zero = 0.0f32.to_bits();
        let mid = 25.0f32.to_bits();
        let huge = f32::MAX.to_bits();
        let thresholds = [small, zero, mid, huge, rng.u32()];
        let flags = [0u8, 1, 0xFF];
        let modes = [0u32, 1, 0xE0, rng.u32() | 1];
        let marks = [0u32, 1, 2, 3, 0x100, rng.u32()];
        let probes = [0u32, 1, 0xFF, 0x100, 0x1FF, u32::MAX, rng.u32()];
        let checks = [0u32, 1, 0xFF, 0x100, 0x1FF, u32::MAX, rng.u32()];
        let mut inputs = Vec::new();
        for &flag in &flags {
            for &mode in &modes {
                for &px in &coords {
                    for &py in &[bare, four, nan, rng.u32()] {
                        for &pz in &[bare, one, rng.u32()] {
                            for &threshold in &thresholds {
                                for &mark in &marks {
                                    inputs.push((flag, mode, [px, py, pz], threshold, mark));
                                }
                            }
                        }
                    }
                }
            }
        }
        let mut picked = Vec::new();
        for (i, input) in inputs.iter().enumerate() {
            if i % 29 == 0 {
                picked.push(*input);
            }
        }
        for _ in 0..300 {
            picked.push(inputs[(rng.next() % inputs.len() as u64) as usize]);
        }
        for (flag, mode, pos_bits, threshold_bits, mark) in picked {
            let probe_ans = probes[(rng.next() % probes.len() as u64) as usize];
            let check_ans = checks[(rng.next() % checks.len() as u64) as usize];
            let sub = U32_EDGE[(rng.next() % U32_EDGE.len() as u64) as usize];
            let kind = rng.u32();
            let table = fake_table(0x20 / 4, CHECK_SLOT, check_addr());
            let ped = ped_blob();
            let mut ped_words = *ped;
            for w in ped_words.iter_mut() {
                *w = rng.u32();
            }
            let ped_boxed = Box::new(ped_words);
            let ped_before = *ped_boxed;
            let ped_addr = if rng.u32() & 1 == 0 {
                0
            } else {
                addr(&ped_boxed[0])
            };
            let mut blob = *flee_blob();
            for w in blob.iter_mut() {
                *w = rng.u32();
            }
            blob[FL_VTABLE] = addr(&table[0]);
            blob[FL_SUB] = sub;
            blob[FL_MARKS] = mark;
            blob[FL_KIND] = kind;
            blob[FL_POS] = pos_bits[0];
            blob[FL_POS + 1] = pos_bits[1];
            blob[FL_POS + 2] = pos_bits[2];
            set_blob_byte(&mut blob, FL_FLAG_BYTE, flag);
            blob[FL_MODE] = mode;
            set_blob_byte(&mut blob, FL_STATE_BYTE, rng.u32() as u8);
            let boxed = Box::new(blob);
            let before = *boxed;
            set_threshold(threshold_bits);
            PROBE_ANS.store(probe_ans, Ordering::SeqCst);
            CHECK_ANS.store(check_ans, Ordering::SeqCst);
            PROBE_COUNT.store(0, Ordering::SeqCst);
            CHECK_COUNT.store(0, Ordering::SeqCst);
            DISP_COUNT.store(0, Ordering::SeqCst);
            let got = unsafe { fn_00DA5160::rw_00da5160(addr(&boxed[0]), ped_addr) };
            // Paths derived from the stub logs and the gate-bit input.
            let probed = PROBE_COUNT.load(Ordering::SeqCst) == 1;
            assert!(PROBE_COUNT.load(Ordering::SeqCst) <= 1);
            if probed {
                assert_eq!(PROBE_TASK.load(Ordering::SeqCst), addr(&boxed[0]));
                assert_eq!(PROBE_PED.load(Ordering::SeqCst), ped_addr);
            }
            let kept_by_probe = probed && probe_ans & 0xFF != 0;
            let want_checks = u32::from(!kept_by_probe && mark & 1 == 0);
            assert_eq!(CHECK_COUNT.load(Ordering::SeqCst), want_checks);
            if want_checks == 1 {
                assert_eq!(CHECK_TASK.load(Ordering::SeqCst), addr(&boxed[0]));
                assert_eq!(CHECK_PED.load(Ordering::SeqCst), ped_addr);
                assert_eq!(CHECK_ONE.load(Ordering::SeqCst), 1);
                assert_eq!(CHECK_ZERO.load(Ordering::SeqCst), 0);
            }
            let refused = want_checks == 1 && check_ans & 0xFF == 0;
            let want_dispatch = u32::from(!kept_by_probe && !refused);
            assert_eq!(DISP_COUNT.load(Ordering::SeqCst), want_dispatch);
            if want_dispatch == 1 {
                assert_eq!(DISP_TASK.load(Ordering::SeqCst), addr(&boxed[0]));
                assert_eq!(DISP_PED.load(Ordering::SeqCst), ped_addr);
            }
            let expect_ret = if kept_by_probe || refused { sub } else { 0 };
            assert_eq!(got, expect_ret);
            let mut expect = before;
            if want_checks == 1 && !refused {
                expect[FL_MARKS] = mark | 2;
            }
            assert_eq!(*boxed, expect);
            assert_eq!(*ped_boxed, ped_before);
            let pos = [
                f32::from_bits(pos_bits[0]),
                f32::from_bits(pos_bits[1]),
                f32::from_bits(pos_bits[2]),
            ];
            let mut lift = FleeTask::new(
                Handle32::new(sub),
                mark,
                kind,
                pos,
                flag != 0,
                Handle32::new(mode),
                blob_byte(&boxed[..], FL_STATE_BYTE),
            );
            let mut fake = Fake {
                probe_answer: probe_ans,
                check_answer: check_ans,
                probes: Vec::new(),
                checks: Vec::new(),
                dispatches: Vec::new(),
            };
            let lift_ret = lift.poll(
                Handle32::new(ped_addr),
                f32::from_bits(threshold_bits),
                &mut fake,
            );
            assert_eq!(Handle32::raw_or_zero(lift_ret), got);
            assert_eq!(fake.probes.len() as u32, u32::from(probed));
            if probed {
                assert_eq!(fake.probes[0], ped_addr);
            }
            assert_eq!(fake.checks.len() as u32, want_checks);
            if want_checks == 1 {
                assert_eq!(fake.checks[0], ped_addr);
            }
            assert_eq!(fake.dispatches.len() as u32, want_dispatch);
            if want_dispatch == 1 {
                assert_eq!(fake.dispatches[0], ped_addr);
            }
            assert_eq!(lift.marks(), boxed[FL_MARKS]);
            assert_eq!(lift.subtask(), Handle32::new(sub));
            let mut wrong = FleeTask::new(
                Handle32::new(sub),
                mark,
                kind,
                pos,
                flag != 0,
                Handle32::new(mode),
                blob_byte(&boxed[..], FL_STATE_BYTE),
            );
            let mut wfake = Fake {
                probe_answer: probe_ans,
                check_answer: check_ans,
                probes: Vec::new(),
                checks: Vec::new(),
                dispatches: Vec::new(),
            };
            let w_ret = wrong_poll(
                &mut wrong,
                Handle32::new(ped_addr),
                f32::from_bits(threshold_bits),
                &mut wfake,
            );
            if Handle32::raw_or_zero(w_ret) != got || wfake.probes.len() as u32 != u32::from(probed)
            {
                caught += 1;
            }
            cases += 1;
        }
        assert!(caught > 0, "wrong flee poll never caught ({cases} cases)");
    }
}
