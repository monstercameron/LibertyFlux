//! Differential case: the duck update slot against its verified rewrite
//! on the same generated inputs.
//!
//! Compares the return value, the full task and ped blobs, and every
//! helper call (slot and arguments in order) against the lift's trait
//! calls, both sides given the same scripted answers. The sample stub
//! lives in a fake ped table, the finish stub in a fake task table. A
//! deliberately wrong lift must be caught at least once. 32-bit only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use core::sync::atomic::{AtomicU32, Ordering};

    use lf_core::Handle32;
    use lf_peds_tasks::tasks::{DuckPed, DuckPedSide, DuckTask, DuckTaskSide};
    use lf_taskdiff::rewrites::*;
    use lf_taskdiff::{set_callee, set_one, set_tick};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        D_DONE_BYTE, D_FLAGGED_BYTE, D_LEVEL, D_MARKS, D_SPAN, D_START, D_TAG_BYTE, D_VTABLE, Rng,
        addr, blob_byte, duck_blob, fake_table, ped_blob, set_blob_byte,
    };

    // Announce stub (slot 1) and its recording.
    static ANN_PED: AtomicU32 = AtomicU32::new(0);
    static ANN_FLAG: AtomicU32 = AtomicU32::new(0);
    static ANN_MASK: AtomicU32 = AtomicU32::new(0);
    static ANN_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn announce_stub(ped: u32, flag: u32, mask: u32) -> u32 {
        ANN_PED.store(ped, Ordering::SeqCst);
        ANN_FLAG.store(flag, Ordering::SeqCst);
        ANN_MASK.store(mask, Ordering::SeqCst);
        ANN_COUNT.fetch_add(1, Ordering::SeqCst);
        0
    }

    // Sustain stub (slot 2) and its recording.
    static SUS_TASK: AtomicU32 = AtomicU32::new(0);
    static SUS_PED: AtomicU32 = AtomicU32::new(0);
    static SUS_Z0: AtomicU32 = AtomicU32::new(0);
    static SUS_Z1: AtomicU32 = AtomicU32::new(0);
    static SUS_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn sustain_stub(task: u32, ped: u32, z0: u32, z1: u32) -> u32 {
        SUS_TASK.store(task, Ordering::SeqCst);
        SUS_PED.store(ped, Ordering::SeqCst);
        SUS_Z0.store(z0, Ordering::SeqCst);
        SUS_Z1.store(z1, Ordering::SeqCst);
        SUS_COUNT.fetch_add(1, Ordering::SeqCst);
        0
    }

    // Elapsed stub (slot 3) and its recording.
    static ELAPSED_ANS: AtomicU32 = AtomicU32::new(0);
    static ELAPSED_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "cdecl" fn elapsed_stub() -> u32 {
        ELAPSED_COUNT.fetch_add(1, Ordering::SeqCst);
        ELAPSED_ANS.load(Ordering::SeqCst)
    }

    // Sample stub (ped slot +0xfc) and its recording.
    static SAMPLE_PED: AtomicU32 = AtomicU32::new(0);
    static SAMPLE_ANS: AtomicU32 = AtomicU32::new(0);
    static SAMPLE_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn sample_stub(ped: u32) -> f32 {
        SAMPLE_PED.store(ped, Ordering::SeqCst);
        SAMPLE_COUNT.fetch_add(1, Ordering::SeqCst);
        f32::from_bits(SAMPLE_ANS.load(Ordering::SeqCst))
    }

    // Finish stub (task slot +0x14) and its recording.
    static FINISH_TASK: AtomicU32 = AtomicU32::new(0);
    static FINISH_PED: AtomicU32 = AtomicU32::new(0);
    static FINISH_ONE: AtomicU32 = AtomicU32::new(0);
    static FINISH_ZERO: AtomicU32 = AtomicU32::new(0);
    static FINISH_ANS: AtomicU32 = AtomicU32::new(0);
    static FINISH_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn finish_stub(task: u32, ped: u32, one: u32, zero: u32) -> u32 {
        FINISH_TASK.store(task, Ordering::SeqCst);
        FINISH_PED.store(ped, Ordering::SeqCst);
        FINISH_ONE.store(one, Ordering::SeqCst);
        FINISH_ZERO.store(zero, Ordering::SeqCst);
        FINISH_COUNT.fetch_add(1, Ordering::SeqCst);
        FINISH_ANS.load(Ordering::SeqCst)
    }

    fn announce_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32, u32) -> u32 = announce_stub;
        f as usize as u32
    }

    fn sustain_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 = sustain_stub;
        f as usize as u32
    }

    fn elapsed_addr() -> u32 {
        let f: extern "cdecl" fn() -> u32 = elapsed_stub;
        f as usize as u32
    }

    fn sample_addr() -> u32 {
        let f: extern "thiscall" fn(u32) -> f32 = sample_stub;
        f as usize as u32
    }

    fn finish_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 = finish_stub;
        f as usize as u32
    }

    struct PedFake {
        sample_answer: f32,
        samples: Vec<u32>,
        announces: Vec<(u32, u32)>,
    }

    impl DuckPedSide for PedFake {
        fn sample(&mut self, ped: Handle32<DuckPed>) -> f32 {
            self.samples.push(ped.get());
            self.sample_answer
        }

        fn announce(&mut self, ped: Handle32<DuckPed>, flag: u32) {
            self.announces.push((ped.get(), flag));
        }
    }

    struct PartsFake {
        elapsed_answer: u32,
        finish_answer: u32,
        sustains: Vec<u32>,
        elapsed_calls: u32,
        finish_calls: Vec<u32>,
    }

    impl DuckTaskSide for PartsFake {
        fn sustain(&mut self, ped: Handle32<DuckPed>) {
            self.sustains.push(ped.get());
        }

        fn elapsed(&mut self) -> u32 {
            self.elapsed_calls += 1;
            self.elapsed_answer
        }

        fn finish_check(&mut self, ped: Handle32<DuckPed>) -> u32 {
            self.finish_calls.push(ped.get());
            self.finish_answer
        }
    }

    /// Samples the threshold edges: zeros, ones, infinities, NaNs and
    /// values around 1.0, each named for what it is.
    fn float_bits() -> Vec<u32> {
        let pos_zero = 0.0f32.to_bits();
        let neg_zero = (-0.0f32).to_bits();
        let one = 1.0f32.to_bits();
        let just_below_one = 0.999_999_94f32.to_bits();
        let just_above_one = 1.000_000_1f32.to_bits();
        let large = 1.0e30f32.to_bits();
        let neg_large = (-1.0e30f32).to_bits();
        let tiny = f32::MIN_POSITIVE.to_bits();
        let pos_inf = f32::INFINITY.to_bits();
        let neg_inf = f32::NEG_INFINITY.to_bits();
        // Quiet NaNs: a positive and a negative payload carrier.
        let nan_a = f32::NAN.to_bits();
        let nan_b = (-f32::NAN).to_bits();
        vec![
            pos_zero,
            neg_zero,
            one,
            just_below_one,
            just_above_one,
            large,
            neg_large,
            tiny,
            pos_inf,
            neg_inf,
            nan_a,
            nan_b,
        ]
    }

    /// Finishes when the sample reaches the limit (the inverted test).
    #[allow(clippy::too_many_arguments)]
    fn wrong_update(
        task: &mut DuckTask,
        ped: Handle32<DuckPed>,
        tick: u32,
        sample_limit: f32,
        peds: &mut PedFake,
        parts: &mut PartsFake,
    ) -> u32 {
        // The timeout arm is shared with the real update.
        if task.span() != 0 && tick.wrapping_sub(task.start()) >= task.span() {
            *task = DuckTask::new(
                task.marks(),
                task.start(),
                task.span(),
                task.level(),
                true,
                task.flagged(),
                task.tag(),
            );
        }
        if !task.done() {
            let sample = peds.sample(ped);
            // Inverted: sustain below the limit, finish at or above it.
            if sample_limit > sample {
                peds.announce(ped, 1);
                if task.flagged() {
                    return 0;
                }
                if task.span() != 0 && tick > task.start().wrapping_add(task.span()) {
                    parts.sustain(ped);
                }
                if task.level() <= 0 {
                    return 0;
                }
                let sub = parts.elapsed();
                let left = task.span().wrapping_sub(sub);
                let clamped = if (left as i32) < 0 { 0 } else { left };
                *task = DuckTask::new(
                    task.marks(),
                    task.start(),
                    task.span(),
                    clamped as u16 as i16,
                    task.done(),
                    task.flagged(),
                    task.tag(),
                );
                return 0;
            }
        }
        if !task.flagged() && task.marks() & 1 == 0 {
            if parts.finish_check(ped) & 0xFF != 0 {
                *task = DuckTask::new(
                    task.marks() | 2,
                    task.start(),
                    task.span(),
                    task.level(),
                    task.done(),
                    task.flagged(),
                    task.tag(),
                );
            }
        }
        peds.announce(ped, 0);
        1
    }

    #[test]
    fn duck_update_matches() {
        // Sample slot byte offset 0xfc, finish slot 0x14, as word indexes.
        const SAMPLE_SLOT: usize = 0xFC / 4;
        const FINISH_SLOT: usize = 0x14 / 4;
        set_callee(1, announce_addr());
        set_callee(2, sustain_addr());
        set_callee(3, elapsed_addr());
        let mut rng = Rng(0xD0C4);
        let mut caught = 0;
        let mut cases = 0;
        // Timeouts: zero, small, large and wrapping spans and starts.
        let spans = [0u32, 1, 2, 100, 0x200, 0x8000_0000, 0xFFFF_FFFF, rng.u32()];
        let starts = [0u32, 1, 1000, 0xFFFF_FF00, 0xFFFF_FFFF, rng.u32()];
        let levels = [-2i16, -1, 0, 1, 5, i16::MIN, i16::MAX];
        let marks = [0u32, 1, 2, 3, 0x100, 0xFFFF_FFFF, rng.u32()];
        let done_bytes = [0u8, 1, 0xFF];
        let flagged_bytes = [0u8, 1, 0xFF];
        let floats = float_bits();
        let mut inputs = Vec::new();
        for &span in &spans {
            for &start in &starts {
                // Ticks: around the deadline plus edges and random.
                let deadline = start.wrapping_add(span);
                let mut ticks = vec![
                    deadline.wrapping_sub(1),
                    deadline,
                    deadline.wrapping_add(1),
                    0,
                    1,
                    u32::MAX,
                    rng.u32(),
                ];
                // A wrapped-timeout shape sustains only through wraparound.
                ticks.push(start.wrapping_add(span.wrapping_sub(1)));
                for &tick in &ticks {
                    for &level in &levels {
                        for &mark in &marks {
                            for &done in &done_bytes {
                                for &flagged in &flagged_bytes {
                                    inputs.push((span, start, tick, level, mark, done, flagged));
                                }
                            }
                        }
                    }
                }
            }
        }
        // Cap the cross product with a stride so the binary stays quick
        // while every axis keeps its edges: run every 37th input plus a
        // random slice. The stride is coprime to every axis length.
        let mut picked = Vec::new();
        for (i, input) in inputs.iter().enumerate() {
            if i % 37 == 0 {
                picked.push(*input);
            }
        }
        for _ in 0..400 {
            picked.push(inputs[(rng.next() % inputs.len() as u64) as usize]);
        }
        for (span, start, tick, level, mark, done, flagged) in picked {
            let sample_bits = floats[(rng.next() % floats.len() as u64) as usize];
            let limit_bits = floats[(rng.next() % floats.len() as u64) as usize];
            let sub_choices = [0u32, 1, span, span.wrapping_add(1), u32::MAX, rng.u32()];
            let sub = sub_choices[(rng.next() % sub_choices.len() as u64) as usize];
            let finish_choices = [0u32, 1, 0x100, 0x1FF, u32::MAX, rng.u32()];
            let finish_ans = finish_choices[(rng.next() % finish_choices.len() as u64) as usize];

            let sample_table = fake_table(0x100 / 4, SAMPLE_SLOT, sample_addr());
            let finish_table = fake_table(0x20 / 4, FINISH_SLOT, finish_addr());
            let mut ped = ped_blob();
            for w in ped.iter_mut() {
                *w = rng.u32();
            }
            ped[0] = addr(&sample_table[0]);
            let ped_before = *ped;
            let ped_addr = addr(&ped[0]);
            let mut blob = *duck_blob();
            for w in blob.iter_mut() {
                *w = rng.u32();
            }
            blob[D_VTABLE] = addr(&finish_table[0]);
            blob[D_MARKS] = mark;
            blob[D_START] = start;
            blob[D_SPAN] = span;
            blob[D_LEVEL] = (blob[D_LEVEL] & 0xFFFF_0000) | u32::from(level as u16);
            set_blob_byte(&mut blob, D_DONE_BYTE, done);
            set_blob_byte(&mut blob, D_FLAGGED_BYTE, flagged);
            set_blob_byte(&mut blob, D_TAG_BYTE, rng.u32() as u8);
            let tag_byte = blob_byte(&blob, D_TAG_BYTE);
            let boxed = Box::new(blob);
            let before = *boxed;
            set_tick(tick);
            set_one(limit_bits);
            ELAPSED_ANS.store(sub, Ordering::SeqCst);
            FINISH_ANS.store(finish_ans, Ordering::SeqCst);
            SAMPLE_ANS.store(sample_bits, Ordering::SeqCst);
            ANN_COUNT.store(0, Ordering::SeqCst);
            SUS_COUNT.store(0, Ordering::SeqCst);
            ELAPSED_COUNT.store(0, Ordering::SeqCst);
            SAMPLE_COUNT.store(0, Ordering::SeqCst);
            FINISH_COUNT.store(0, Ordering::SeqCst);
            let got = unsafe { fn_00D4E1C0::rw_00d4e1c0(addr(&boxed[0]), ped_addr) };

            // Expectations derived from the case inputs alone.
            let sample = f32::from_bits(sample_bits);
            let limit = f32::from_bits(limit_bits);
            let timeout = span != 0 && tick.wrapping_sub(start) >= span;
            let done_after = done != 0 || timeout;
            let sustain_path = !done_after && !(limit > sample);
            let tag = || {
                // Short label for assertion messages.
                (
                    span,
                    start,
                    tick,
                    level,
                    mark,
                    done,
                    flagged,
                    sample_bits,
                    limit_bits,
                )
            };
            assert_eq!(
                SAMPLE_COUNT.load(Ordering::SeqCst),
                u32::from(!done_after),
                "sample count {:?}",
                tag()
            );
            if !done_after {
                assert_eq!(SAMPLE_PED.load(Ordering::SeqCst), ped_addr);
            }
            assert_eq!(
                ANN_COUNT.load(Ordering::SeqCst),
                1,
                "announce count {:?}",
                tag()
            );
            assert_eq!(ANN_PED.load(Ordering::SeqCst), ped_addr);
            assert_eq!(
                ANN_FLAG.load(Ordering::SeqCst),
                u32::from(sustain_path),
                "announce flag {:?}",
                tag()
            );
            assert_eq!(ANN_MASK.load(Ordering::SeqCst), 0xFFFF_FFFF);
            let want_sustain =
                sustain_path && flagged == 0 && span != 0 && tick > start.wrapping_add(span);
            assert_eq!(
                SUS_COUNT.load(Ordering::SeqCst),
                u32::from(want_sustain),
                "sustain count {:?}",
                tag()
            );
            if want_sustain {
                assert_eq!(SUS_TASK.load(Ordering::SeqCst), addr(&boxed[0]));
                assert_eq!(SUS_PED.load(Ordering::SeqCst), ped_addr);
                assert_eq!(SUS_Z0.load(Ordering::SeqCst), 0);
                assert_eq!(SUS_Z1.load(Ordering::SeqCst), 0);
            }
            let want_elapsed = sustain_path && flagged == 0 && level > 0;
            assert_eq!(
                ELAPSED_COUNT.load(Ordering::SeqCst),
                u32::from(want_elapsed),
                "elapsed count {:?}",
                tag()
            );
            let want_finish = !sustain_path && flagged == 0 && mark & 1 == 0;
            assert_eq!(
                FINISH_COUNT.load(Ordering::SeqCst),
                u32::from(want_finish),
                "finish count {:?}",
                tag()
            );
            if want_finish {
                assert_eq!(FINISH_TASK.load(Ordering::SeqCst), addr(&boxed[0]));
                assert_eq!(FINISH_PED.load(Ordering::SeqCst), ped_addr);
                assert_eq!(FINISH_ONE.load(Ordering::SeqCst), 1);
                assert_eq!(FINISH_ZERO.load(Ordering::SeqCst), 0);
            }
            assert_eq!(got, u32::from(!sustain_path), "return {:?}", tag());
            // The expected image: done byte, level half and marks word.
            let mut expect = before;
            if timeout {
                set_blob_byte(&mut expect, D_DONE_BYTE, 1);
            }
            if want_elapsed {
                let left = span.wrapping_sub(sub);
                let clamped = if (left as i32) < 0 { 0 } else { left };
                expect[D_LEVEL] = (expect[D_LEVEL] & 0xFFFF_0000) | (clamped & 0xFFFF);
            }
            if want_finish && finish_ans & 0xFF != 0 {
                expect[D_MARKS] = mark | 2;
            }
            assert_eq!(*boxed, expect, "blob {:?}", tag());
            assert_eq!(*ped, ped_before, "ped {:?}", tag());

            // The lift on the same inputs, with the same scripted answers.
            let ped_handle = Handle32::new(ped_addr).unwrap();
            let mut lift =
                DuckTask::new(mark, start, span, level, done != 0, flagged != 0, tag_byte);
            let mut peds = PedFake {
                sample_answer: sample,
                samples: Vec::new(),
                announces: Vec::new(),
            };
            let mut parts = PartsFake {
                elapsed_answer: sub,
                finish_answer: finish_ans,
                sustains: Vec::new(),
                elapsed_calls: 0,
                finish_calls: Vec::new(),
            };
            let lift_ret = lift.update(ped_handle, tick, limit, &mut peds, &mut parts);
            assert_eq!(lift_ret, got, "lift return {:?}", tag());
            assert_eq!(peds.samples.len() as u32, u32::from(!done_after));
            if !done_after {
                assert_eq!(peds.samples[0], ped_addr);
            }
            assert_eq!(peds.announces, vec![(ped_addr, u32::from(sustain_path))]);
            assert_eq!(parts.sustains.len() as u32, u32::from(want_sustain));
            assert_eq!(parts.elapsed_calls, u32::from(want_elapsed));
            assert_eq!(parts.finish_calls.len() as u32, u32::from(want_finish));
            assert_eq!(lift.marks(), boxed[D_MARKS], "lift marks {:?}", tag());
            assert_eq!(
                lift.done(),
                blob_byte(&boxed[..], D_DONE_BYTE) != 0,
                "lift done {:?}",
                tag()
            );
            assert_eq!(
                lift.flagged(),
                blob_byte(&boxed[..], D_FLAGGED_BYTE) != 0,
                "lift flagged {:?}",
                tag()
            );
            assert_eq!(
                lift.level(),
                (boxed[D_LEVEL] & 0xFFFF) as u16 as i16,
                "lift level {:?}",
                tag()
            );
            assert_eq!(lift.span(), span);
            assert_eq!(lift.start(), start);

            // The inverted sample test must disagree wherever the sample runs.
            let mut wrong =
                DuckTask::new(mark, start, span, level, done != 0, flagged != 0, tag_byte);
            let mut wpeds = PedFake {
                sample_answer: sample,
                samples: Vec::new(),
                announces: Vec::new(),
            };
            let mut wparts = PartsFake {
                elapsed_answer: sub,
                finish_answer: finish_ans,
                sustains: Vec::new(),
                elapsed_calls: 0,
                finish_calls: Vec::new(),
            };
            let w_ret = wrong_update(&mut wrong, ped_handle, tick, limit, &mut wpeds, &mut wparts);
            let stub_flag = ANN_FLAG.load(Ordering::SeqCst);
            if w_ret != got || wpeds.announces.len() != 1 || wpeds.announces[0].1 != stub_flag {
                caught += 1;
            }
            cases += 1;
        }
        assert!(caught > 0, "wrong duck update never caught ({cases} cases)");
    }
}
