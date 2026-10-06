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
        D_DONE_BYTE, D_FLAGGED_BYTE, D_LEVEL, D_MARKS, D_START, D_SPAN, D_TAG_BYTE, D_VTABLE, Rng,
        U32_EDGE, addr, blob_byte, duck_blob, fake_table, ped_blob, set_blob_byte,
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
        let mut rng = Rng(0xD0CU);
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
        for (
...[truncated 9059 chars]