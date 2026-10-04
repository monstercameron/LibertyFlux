// original: 0x00a1d560 cam_dirty_recalc_pose (proposed)

/// Recalculates the pose through the worker when the dirty bit is set.
///
/// When the dirty bit in the flag at `+FLAG_OFF` is clear the function
/// returns at once. Otherwise the worker callee runs on a frame slot plus
/// the sums `x1 = [this+K0_OFF] + a1` and `x0 = [this+K1_OFF]` (which are
/// also stored to `+S0_OFF`/`+S1_OFF` first); its returned pointer's four
/// words are copied to `a0[0..16]`, and the dirty bit is cleared. (The
/// frame slot stands in for the original's aligned scratch slot, whose
/// address the call comparison skips; the callee's answer and the copied
/// words are still fully observed.) Returns nothing.
///
/// Original: 0x00a1d560 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00a1d560(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const C_WORKER: u32 = 1;
        const FLAG_OFF: u32 = 0x38d;
        const K0_OFF: u32 = 0x324;
        const K1_OFF: u32 = 0x328;
        const S0_OFF: u32 = 0x304;
        const S1_OFF: u32 = 0x308;
        const DIRTY_BIT: u8 = 8;
        unsafe fn rdf(x: u32) -> f32 {
            unsafe { f32::from_bits((x as *const u32).read_unaligned()) }
        }
        let flag = (this + FLAG_OFF) as *mut u8;
        if flag.read() & DIRTY_BIT == 0 {
            return 0;
        }
        let x1 = core::hint::black_box(rdf(this + K0_OFF)) + core::hint::black_box(f32::from_bits(a1));
        let x0 = rdf(this + K1_OFF);
        ((this + S0_OFF) as *mut u32).write_unaligned(x1.to_bits());
        ((this + S1_OFF) as *mut u32).write_unaligned(x0.to_bits());
        let mut slot: u32 = 0;
        let p = lf_checker_rt::callee_cdecl!(
            C_WORKER, u32, &mut slot as *mut u32 as u32, x1.to_bits(), x0.to_bits()
        );
        for i in 0..4u32 {
            let w = ((p + i * 4) as *const u32).read_unaligned();
            ((a0 + i * 4) as *mut u32).write_unaligned(w);
        }
        flag.write(flag.read() & !DIRTY_BIT);
        0
    }
});
