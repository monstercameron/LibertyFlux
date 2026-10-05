// original: 0x008786E0 rage::crmtComposerOptimized::CombiningFilter::vf4

/// Fold a helper answer into a combining filter's accumulator.
///
/// `this` points to a combining-filter object with a 32-bit accumulator at
/// `+0x30`. A helper runs first (through the patched call slot); its
/// answer is rotated left by 16 bits and xored into the accumulator, and
/// the new accumulator value is returned.
///
/// Original: 0x008786E0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_008786E0(this: u32) -> u32 {
    unsafe {
        const ACC_OFF: u32 = 0x30;
        const ROT_BITS: u32 = 16;
        const HELPER: u32 = 1;
        let answer = lf_checker_rt::callee_thiscall!(HELPER, u32, this);
        let folded = answer.rotate_left(ROT_BITS);
        let acc = (this.wrapping_add(ACC_OFF) as *const u32).read_unaligned();
        acc ^ folded
    }
});
