// original: 0x005ae630 gated_accumulator_update
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global};

/// Update one gated accumulator from the tagged sample provider.
///
/// When the enable byte at offset 2 is clear the accumulator at offset 4 is
/// reset to zero. Otherwise samples are combined through one scratch slot:
/// the first sample scaled by the shared counter, the second sample added
/// on top, and a gate sample that resets the accumulator when it strictly
/// exceeds the running total. Past the gate, a selector triple is resolved
/// and its verdict gates two bounded adjustment stages that add or subtract
/// a globally scaled step. Returns nothing meaningful.
///
/// The sample buffers are filled by the callee but only the returned sample
/// pair is ever read; the three later calls reuse the earlier buffers'
/// slots, mirrored here.
export!(thiscall, rw_005ae630(this: u32) -> u32 {
    const FRAME_COUNT: u32 = 0x0118F4C0;
    unsafe {
        let base = this as *mut u8;
        if base.add(2).read() == 0 {
            (base.add(4) as *mut u32).write(0);
            return 0;
        }
        let acc = base.add(4) as *mut u32;
        let count = global::<u32>(FRAME_COUNT).read();
        // Stage 1: first sample scaled by the shared counter.
        let mut buf1 = [0u32; 2];
        let v1 = callee_cdecl!(1, u32, buf1.as_mut_ptr() as u32, 0x4b);
        let scale = (count.wrapping_add(1) as i32) as f32;
        let s1 = f32::from_bits(((v1.wrapping_add(4)) as *const u32).read());
        let mut slot = s1 * scale;
        // Stage 2: second sample added onto the running total.
        let mut buf2 = [0u32; 2];
        let v2 = callee_cdecl!(1, u32, buf2.as_mut_ptr() as u32, 0x48);
        let s2 = f32::from_bits(((v2.wrapping_add(4)) as *const u32).read());
        slot = s2 + slot;
        // Stage 3: gate sample resets the accumulator when strictly above.
        let mut buf3 = [0u32; 2];
        let v3 = callee_cdecl!(1, u32, buf3.as_mut_ptr() as u32, 0x16);
        let gate = f32::from_bits((v3 as *const u32).read());
        if gate > slot {
            acc.write(0);
            return 0;
        }
        // Selector triple; its verdict becomes the adjustment gate.
        let r1 = callee_cdecl!(2, u32, 1);
        let r2 = callee_thiscall!(3, u32, r1, 0);
        let r3 = callee_cdecl!(4, u32, r2);
        let gate_flag: u32 = if r3 == 0 { 1 } else { 0 };
        // Stage 4: difference sample plus the live accumulator.
        let v4 = callee_cdecl!(1, u32, buf3.as_mut_ptr() as u32, 0x16);
        let v5 = callee_cdecl!(1, u32, buf2.as_mut_ptr() as u32, 0x48);
        let d0 = f32::from_bits((v4 as *const u32).read());
        let d1 = f32::from_bits(((v5.wrapping_add(4)) as *const u32).read());
        slot = (d0 - d1) + f32::from_bits(acc.read());
        let v6 = callee_cdecl!(1, u32, buf1.as_mut_ptr() as u32, 0x4b);
        let count2 = global::<u32>(FRAME_COUNT).read();
        let scale2 = (count2.wrapping_add(1) as i32) as f32;
        let s6 = f32::from_bits(((v6.wrapping_add(4)) as *const u32).read());
        // jbe skips the raise stage when the product is at/below the slot
        // or unordered.
        if !(scale2 * s6 > slot) {
            return lower_stage(acc, gate_flag);
        }
        // Raise stage: polled value above 10 applies unconditionally,
        // otherwise a 7-way check plus the gate flag is required.
        let p1 = callee_cdecl!(2, u32, 1);
        let q1 = callee_cdecl!(4, u32, p1.wrapping_add(0x2b48));
        let mut raised = (q1 as i32) > 10;
        if !raised {
            let al = callee_cdecl!(5, u32, 1, 0, 0, 0, 1, 0, 1);
            raised = (al as u8) != 0 && gate_flag != 0;
        }
        if raised {
            let step = step_size();
            acc.write((f32::from_bits(acc.read()) + step).to_bits());
        }
        lower_stage(acc, gate_flag)
    }
});

/// Lower stage: when the accumulator exceeds its limit, a polled value
/// below -10 subtracts the step unconditionally, otherwise a 7-way check
/// plus the gate flag is required.
#[inline(always)]
fn lower_stage(acc: *mut u32, gate_flag: u32) -> u32 {
    const LIMIT: u32 = 0x00FE8628;
    unsafe {
        let limit = f32::from_bits(global::<u32>(LIMIT).read());
        // jbe returns when at/below the limit or unordered.
        if !(f32::from_bits(acc.read()) > limit) {
            return 0;
        }
        let p2 = callee_cdecl!(2, u32, 1);
        let q2 = callee_cdecl!(4, u32, p2.wrapping_add(0x2b48));
        let mut lowered = (q2 as i32) < -10;
        if !lowered {
            let al = callee_cdecl!(5, u32, 0, 0, 0, 0, 1, 0, 1);
            lowered = (al as u8) != 0 && gate_flag != 0;
        }
        if lowered {
            let step = step_size();
            acc.write((f32::from_bits(acc.read()) - step).to_bits());
        }
        0
    }
}

/// Globally scaled adjustment step (product of the two scale factors).
#[inline(always)]
fn step_size() -> f32 {
    const STEP_A: u32 = 0x0117359C;
    const STEP_B: u32 = 0x00FE87D0;
    unsafe {
        let a = f32::from_bits(global::<u32>(STEP_A).read());
        let b = f32::from_bits(global::<u32>(STEP_B).read());
        a * b
    }
}
