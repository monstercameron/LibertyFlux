// original: 0x0066DB10 rage::sysTimeManager::vf1

/// Reset a system time manager to its default rates.
///
/// `this` points to the manager. The reset writes a fixed set of fields and
/// reads nothing: the enable flag at `+0x30` becomes 1, the frame-step field
/// at `+0x08` becomes 0x3C888889 (about 1/60), the rate field at `+0x0C`
/// becomes 0x42700000 (60.0), the three accumulator/counter fields at `+0x10`,
/// `+0x14` and `+0x28` become 0, and the scale field at `+0x4C` becomes
/// 0x3F800000 (1.0). The return register is untouched, so the caller sees
/// whatever it held on entry; the contract does not compare it.
///
/// Original: 0x0066DB10 (thiscall, no stack arguments, no calls).
lf_checker_rt::export!(thiscall, rw_0066db10(this: u32) -> u32 {
    unsafe {
        const ENABLED: u32 = 0x30;
        const FRAME_STEP: u32 = 0x08;
        const RATE: u32 = 0x0c;
        const ACCUM_A: u32 = 0x10;
        const ACCUM_B: u32 = 0x14;
        const ACCUM_C: u32 = 0x28;
        const SCALE: u32 = 0x4c;
        const STEP_BITS: u32 = 0x3c88_8889;
        const RATE_BITS: u32 = 0x4270_0000;
        const SCALE_BITS: u32 = 0x3f80_0000;

        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        ((this + ENABLED) as *mut u8).write_unaligned(1);
        wr32(this + FRAME_STEP, STEP_BITS);
        wr32(this + RATE, RATE_BITS);
        wr32(this + ACCUM_B, 0);
        wr32(this + ACCUM_A, 0);
        wr32(this + ACCUM_C, 0);
        wr32(this + SCALE, SCALE_BITS);
        0
    }
});
