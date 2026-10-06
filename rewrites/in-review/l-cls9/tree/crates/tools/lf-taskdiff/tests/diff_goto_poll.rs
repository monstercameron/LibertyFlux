//! Differential case: the goto periodic update slot against its verified
//! rewrite on the same generated inputs.
//!
//! Compares the return value, the full task, subtask and ped blobs, and
//! the probe, fallback and dispatch calls against the lift's trait
//! calls, both sides given the same scripted answers. The subtask check
//! stub lives in a fake subtask table; the null-subtask cases run the
//! probe-keeps path only. A deliberately wrong lift must be caught at
//! least once. 32-bit only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use core::sync::atomic::{AtomicU32, Ordering};

    use lf_core::Handle32;
    use lf_peds_tasks::tasks::{GotoPed, GotoPoll, GotoTask, SubTask, SubVerdict};
    use lf_taskdiff::rewrites::*;
    use lf_taskdiff::{set_callee, set_threshold, set_tick};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        G_ARMED_BYTE, G_FLAG_BYTE, G_KIND, G_MODE, G_POS, G_RESTAMP_BYTE, G_STAMP, G_SUB, G_WAITCP, Rng,
        addr, blob_byte, fake_table, goto_blob, ped_blob, query_blob, set_blob_byte,
    };

    /// The fallback table word the proof pins.
    const FALLBACK_TABLE: u32 = 0x00EEF598;
    /// The fallback blend word the proof pins.
    const FALLBACK_BLEND: u32 = 0x3D4C_CCCD;

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

    // Fallback stub (slot 4) and its recording.
    static FALL_PED: AtomicU32 = AtomicU32::new(0);
    static FALL_TABLE: AtomicU32 = AtomicU32::new(0);
    static FALL_BLEND: AtomicU32 = AtomicU32::new(0);
    static FALL_Z0: AtomicU32 = AtomicU32::new(0);
    static FALL_Z1: AtomicU32 = AtomicU32::new(0);
    static FALL_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn fallback_stub(ped: u32, table: u32, blend: u32, z0: u32, z1: u32) -> u32 {
        FALL_PED.store(ped, Ordering::SeqCst);
        FALL_TABLE.store(table, Ordering::SeqCst);
        FALL_BLEND.store(blend, Ordering::SeqCst);
        FALL_Z0.store(z0, Ordering::SeqCst);
        FALL_Z1.store(z1, Ordering::SeqCst);
        FALL_COUNT.fetch_add(1, Ordering::SeqCst);
        0
    }

    // Subtask check stub (subtask slot +0x14) and its recording.
    static CHECK_SUB: AtomicU32 = AtomicU32::new(0);
    static CHECK_PED: AtomicU32 = AtomicU32::new(0);
    static CHECK_ONE: AtomicU32 = AtomicU32::new(0);
    static CHECK_ZERO: AtomicU32 = AtomicU32::new(0);
    static CHECK_ANS: AtomicU32 = AtomicU32::new(0);
    static CHECK_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn check_stub(sub: u32, ped: u32, one: u32, zero: u32) -> u32 {
        CHECK_SUB.store(sub, Ordering::SeqCst);
        CHECK_PED.store(ped, Ordering::SeqCst);
        CHECK_ONE.store(one, Ordering::SeqCst);
        CHECK_ZERO.store(zero, Ordering::SeqCst);
        CHECK_COUNT.fetch_add(1, Ordering::SeqCst);
        CHECK_ANS.load(Ordering::SeqCst)
    }

    fn probe_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32) -> u32 = probe_stub;
        f as usize as u32
    }

    fn dispatch_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32) -> u32 = dispatch_stub;
        f as usize as u32
    }

    fn fallback_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 = fallback_stub;
        f as usize as u32
    }

    fn check_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 = check_stub;
        f as usize as u32
    }

    struct Fake {
        probe_answer: u32,
        verdict: SubVerdict,
        probes: Vec<u32>,
        fallbacks: Vec<u32>,
        checks: Vec<(u32, u32)>,
        dispatches: Vec<u32>,
    }

    impl GotoPoll for Fake {
        fn probe(&mut self, ped: Option<Handle32<GotoPed>>) -> u32 {
            self.probes.push(Handle32::raw_or_zero(ped));
            self.probe_answer
        }

        fn fallback(&mut self, ped: Option<Handle32<GotoPed>>) {
            self.fallbacks.push(Handle32::raw_or_zero(ped));
        }

        fn check_subtask(
            &mut self,
            sub: Handle32<SubTask>,
            ped: Option<Handle32<GotoPed>>,
        ) -> SubVerdict {
            self.checks.push((sub.get(), Handle32::raw_or_zero(ped)));
            self.verdict
        }

        fn dispatch(&mut self, ped: Option<Handle32<GotoPed>>) {
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
    /// The timer pre-gate is shared with the real update.
    fn wrong_poll(
        task: &mut GotoTask,
        ped: Option<Handle32<GotoPed>>,
        tick: u32,
        threshold: f32,
        poll: &mut Fake,
    ) -> Option<Handle32<SubTask>> {
        let mut skip_gate = false;
        if task.armed() {
            if task.restamp() {
                *task = GotoTask::new(
                    task.subtask(),
                    task.kind(),
                    task.pos(),
                    task.flag(),
                    task.mode(),
                    task.wait(),
                    tick,
                    task.wait_copy(),
                    task.armed(),
                    false,
                    task.speed(),
                );
            }
            if task.stamp().wrapping_add(task.wait_copy()) <= tick {
                skip_gate = true;
            }
        }
        let pos = task.pos();
        let live = if task.flag() {
            task.mode().is_some()
        } else if task.mode().is_some() {
            false
        } else {
            let sq = add(add(mul(pos[0], pos[0]), mul(pos[1], pos[1])), mul(pos[2], pos[2]));
            sq > threshold
        };
        if !skip_gate && !live {
            if poll.probe(ped) & 0xFF != 0 {
                poll.fallback(ped);
                return task.subtask();
            }
        }
        let Some(sub) = task.subtask() else {
            panic!("wrong poll without a subtask");
        };
        if poll.check_subtask(sub, ped) == SubVerdict::Refused {
            poll.fallback(ped);
            return task.subtask();
        }
        poll.dispatch(ped);
        None
    }

    #[test]
    fn goto_poll_matches() {
        // Check slot byte offset 0x14, as a word index.
        const CHECK_SLOT: usize = 0x14 / 4;
        set_callee(1, probe_addr());
        // Slot 2 is unused by this method; 3 dispatches, 4 falls back.
        set_callee(3, dispatch_addr());
        set_callee(4, fallback_addr());
        let mut rng = Rng(0x6070);
        let mut caught = 0;
        let mut cases = 0;
        let bare = 0.0f32.to_bits();
        let one = 1.0f32.to_bits();
        let three = 3.0f32.to_bits();
        let four = 4.0f32.to_bits();
        let nan = f32::NAN.to_bits();
        let coords = [bare, one, three, four, nan, rng.u32()];
        let thresholds = [0.0f32.to_bits(), 0.05f32.to_bits(), 25.0f32.to_bits(), rng.u32()];
        let flags = [0u8, 1, 0xFF];
        let modes = [0u32, 1, 0xE0, rng.u32() | 1];
        let armed = [0u8, 1, 0xFF];
        let restamps = [0u8, 1, 0xFF];
        let stamps = [0u32, 1, 1000, 0xFFFF_FF00, u32::MAX, rng.u32()];
        let waits = [0u32, 1, 5000, u32::MAX, rng.u32()];
        let probes = [0u32, 1, 0xFF, 0x100, u32::MAX, rng.u32()];
        let checks = [0u32, 1, 0xFF, 0x100, u32::MAX, rng.u32()];
        let sub_marks = [0u32, 1, 2, 3, 0x100, rng.u32()];
        let mut inputs = Vec::new();
        for &is_armed in &armed {
            for &restamp in &restamps {
                for &stamp in &stamps {
                    for &wait in &waits {
                        // Ticks around the deadline (restamps refresh it).
                        let base = if restamp != 0 { 2000u32 } else { stamp };
                        let deadline = base.wrapping_add(wait);
                        let ticks = [
                            deadline.wrapping_sub(1),
                            deadline,
                            deadline.wrapping_add(1),
                            0,
                            u32::MAX,
                            rng.u32(),
                        ];
                        for &tick in &ticks {
                            for &flag in &flags {
                                for &mode in &modes {
                                    inputs.push((
                                        is_armed, restamp, stamp, wait, tick, flag, mode,
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
        let mut picked = Vec::new();
        for (i, input) in inputs.iter().enumerate() {
            if i % 31 == 0 {
                picked.push(*input);
            }
        }
        for _ in 0..300 {
            picked.push(inputs[(rng.next() % inputs.len() as u64) as usize]);
        }
        for (is_armed, restamp, stamp, wait, tick, flag, mode) in picked {
            let probe_ans = probes[(rng.next() % probes.len() as u64) as usize];
            let check_ans = checks[(rng.next() % checks.len() as u64) as usize];
            let sub_mark = sub_marks[(rng.next() % sub_marks.len() as u64) as usize];
            let px = coords[(rng.next() % coords.len() as u64) as usize];
            let py = coords[(rng.next() % coords.len() as u64) as usize];
            let pz = coords[(rng.next() % coords.len() as u64) as usize];
            let threshold_bits =
                thresholds[(rng.next() % thresholds.len() as u64) as usize];
            // Null subtasks only run the probe-keeps path (the only one
            // that never touches the subtask): force a live gate, a
            // firing probe and a disarmed timer, else draw a live one.
            let null_sub = rng.u32() & 7 == 0;
            let (is_armed, restamp, flag, mode, probe_ans) = if null_sub {
                (0, 0, 1, mode | 1, probe_ans & 0xFFFF_FF00 | 1)
            } else {
                (is_armed, restamp, flag, mode, probe_ans)
            };
            let pos_bits = [px, py, pz];
            let kind = rng.u32();
            let table = fake_table(0x20 / 4, CHECK_SLOT, check_addr());
            let ped = ped_blob();
            let mut ped_words = *ped;
            for w in ped_words.iter_mut() {
                *w = rng.u32();
            }
            let ped_boxed = Box::new(ped_words);
            let ped_before = *ped_boxed;
            let ped_addr = if rng.u32() & 1 == 0 { 0 } else { addr(&ped_boxed[0]) };
            // The subtask blob the check sequence reads and marks:
            // table at word 0, marks at word 3 (+0x0c).
            let mut sub_blob = *query_blob();
            for w in sub_blob.iter_mut() {
                *w = rng.u32();
            }
            sub_blob[0] = addr(&table[0]);
            sub_blob[3] = sub_mark;
            let sub_boxed = Box::new(sub_blob);
            let sub_before = *sub_boxed;
            let sub_addr = if null_sub { 0 } else { addr(&sub_boxed[0]) };
            let mut blob = *goto_blob();
            for w in blob.iter_mut() {
                *w = rng.u32();
            }
            blob[G_SUB] = sub_addr;
            blob[G_KIND] = kind;
            blob[G_POS] = pos_bits[0];
            blob[G_POS + 1] = pos_bits[1];
            blob[G_POS + 2] = pos_bits[2];
            set_blob_byte(&mut blob, G_FLAG_BYTE, flag);
            blob[G_MODE] = mode;
            blob[G_STAMP] = stamp;
            blob[G_WAITCP] = wait;
            set_blob_byte(&mut blob, G_ARMED_BYTE, is_armed);
            set_blob_byte(&mut blob, G_RESTAMP_BYTE, restamp);
            let boxed = Box::new(blob);
            let before = *boxed;
            set_tick(tick);
            set_threshold(threshold_bits);
            PROBE_ANS.store(probe_ans, Ordering::SeqCst);
            CHECK_ANS.store(check_ans, Ordering::SeqCst);
            PROBE_COUNT.store(0, Ordering::SeqCst);
            CHECK_COUNT.store(0, Ordering::SeqCst);
            FALL_COUNT.store(0, Ordering::SeqCst);
            DISP_COUNT.store(0, Ordering::SeqCst);
            let got = unsafe { fn_00DA51F0::rw_00da51f0(addr(&boxed[0]), ped_addr) };
            // The timer path from the inputs alone.
            let restamped = is_armed != 0 && restamp != 0;
            let eff_stamp = if restamped { tick } else { stamp };
            let skip_gate = is_armed != 0 && eff_stamp.wrapping_add(wait) <= tick;
            // Whether the gate fired is read off the probe stub: the
            // length arm needs floats, so only the stub decides those.
            let probed = PROBE_COUNT.load(Ordering::SeqCst) == 1;
            assert!(PROBE_COUNT.load(Ordering::SeqCst) <= 1);
            assert!(!skip_gate || !probed);
            if probed {
                assert_eq!(PROBE_TASK.load(Ordering::SeqCst), addr(&boxed[0]));
                assert_eq!(PROBE_PED.load(Ordering::SeqCst), ped_addr);
            }
            let kept_by_probe = probed && probe_ans & 0xFF != 0;
            // The verdict the fake must mirror.
            let verdict = if sub_mark & 1 != 0 || check_ans & 0xFF != 0 {
                SubVerdict::Passed
            } else {
                SubVerdict::Refused
            };
            let want_check_stub = !kept_by_probe && !null_sub && sub_mark & 1 == 0;
            assert_eq!(
                CHECK_COUNT.load(Ordering::SeqCst),
                u32::from(want_check_stub)
            );
            if want_check_stub {
                assert_eq!(CHECK_SUB.load(Ordering::SeqCst), sub_addr);
                assert_eq!(CHECK_PED.load(Ordering::SeqCst), ped_addr);
                assert_eq!(CHECK_ONE.load(Ordering::SeqCst), 1);
                assert_eq!(CHECK_ZERO.load(Ordering::SeqCst), 0);
            }
            let refused = !kept_by_probe && verdict == SubVerdict::Refused;
            let want_fallback = kept_by_probe || refused;
            assert_eq!(FALL_COUNT.load(Ordering::SeqCst), u32::from(want_fallback));
            if want_fallback {
                assert_eq!(FALL_PED.load(Ordering::SeqCst), ped_addr);
                assert_eq!(FALL_TABLE.load(Ordering::SeqCst), FALLBACK_TABLE);
                assert_eq!(FALL_BLEND.load(Ordering::SeqCst), FALLBACK_BLEND);
                assert_eq!(FALL_Z0.load(Ordering::SeqCst), 0);
                assert_eq!(FALL_Z1.load(Ordering::SeqCst), 0);
            }
            let want_dispatch = !kept_by_probe && !refused;
            assert_eq!(DISP_COUNT.load(Ordering::SeqCst), u32::from(want_dispatch));
            if want_dispatch {
                assert_eq!(DISP_TASK.load(Ordering::SeqCst), addr(&boxed[0]));
                assert_eq!(DISP_PED.load(Ordering::SeqCst), ped_addr);
            }
            let expect_ret = if kept_by_probe || refused { sub_addr } else { 0 };
            assert_eq!(got, expect_ret);
            let mut expect = before;
            if restamped {
                expect[G_STAMP] = tick;
                set_blob_byte(&mut expect, G_RESTAMP_BYTE, 0);
            }
            assert_eq!(*boxed, expect);
            let mut expect_sub = sub_before;
            if want_check_stub && check_ans & 0xFF != 0 {
                expect_sub[3] = sub_mark | 2;
            }
            if !null_sub {
                assert_eq!(*sub_boxed, expect_sub);
            }
            assert_eq!(*ped_boxed, ped_before);
            // The lift on the same inputs, with the same scripted answers.
            let pos = [
                f32::from_bits(pos_bits[0]),
                f32::from_bits(pos_bits[1]),
                f32::from_bits(pos_bits[2]),
            ];
            let mut lift = GotoTask::new(
                Handle32::new(sub_addr),
                kind,
                pos,
                flag != 0,
                Handle32::new(mode),
                0,
                stamp,
                wait,
                is_armed != 0,
                restamp != 0,
                0.0,
            );
            let mut fake = Fake {
                probe_answer: probe_ans,
                verdict,
                probes: Vec::new(),
                fallbacks: Vec::new(),
                checks: Vec::new(),
                dispatches: Vec::new(),
            };
            let lift_ret = lift.poll(
                Handle32::new(ped_addr),
                tick,
                f32::from_bits(threshold_bits),
                &mut fake,
            );
            assert_eq!(Handle32::raw_or_zero(lift_ret), got);
            assert_eq!(fake.probes.len() as u32, u32::from(probed));
            if probed {
                assert_eq!(fake.probes[0], ped_addr);
            }
            assert_eq!(fake.fallbacks.len() as u32, u32::from(want_fallback));
            if want_fallback {
                assert_eq!(fake.fallbacks[0], ped_addr);
            }
            // The lift asks the subtask check on every non-kept path; the
            // slot stub only fires past the gate bit.
            assert_eq!(fake.checks.len() as u32, u32::from(!kept_by_probe));
            if !kept_by_probe {
                assert_eq!(fake.checks[0], (sub_addr, ped_addr));
            }
            assert_eq!(fake.dispatches.len() as u32, u32::from(want_dispatch));
            assert_eq!(lift.stamp(), boxed[G_STAMP]);
            assert_eq!(
                lift.restamp(),
                blob_byte(&boxed[..], G_RESTAMP_BYTE) != 0
            );
            // The inverted gate must disagree wherever probing decides.
            if !null_sub {
                let mut wrong = GotoTask::new(
                    Handle32::new(sub_addr),
                    kind,
                    pos,
                    flag != 0,
                    Handle32::new(mode),
                    0,
                    stamp,
                    wait,
                    is_armed != 0,
                    restamp != 0,
                    0.0,
                );
                let mut wfake = Fake {
                    probe_answer: probe_ans,
                    verdict,
                    probes: Vec::new(),
                    fallbacks: Vec::new(),
                    checks: Vec::new(),
                    dispatches: Vec::new(),
                };
                let w_ret = wrong_poll(
                    &mut wrong,
                    Handle32::new(ped_addr),
                    tick,
                    f32::from_bits(threshold_bits),
                    &mut wfake,
                );
                if Handle32::raw_or_zero(w_ret) != got
                    || wfake.probes.len() as u32 != u32::from(probed)
                {
                    caught += 1;
                }
            }
            cases += 1;
        }
        assert!(caught > 0, "wrong goto poll never caught ({cases} cases)");
    }
}