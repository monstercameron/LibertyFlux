// original: 0x0090a2e0 font_range_sample (proposed)
/// Sample a value into the range when the cursor is usable, then advance it.
///
/// `this` points to the range object (`+4` the low bound, `+8` the high
/// bound, `+0xc` the cursor, all signed). The sampler callee is called with
/// the first four stack words when the cursor lies within `[lo, hi]` (both
/// signed comparisons, edges inclusive) or, outside it, when `lo` is
/// negative (signed `jns` fall-through); otherwise nothing is sampled. The cursor is
/// incremented in both cases. Returns 1 when sampled, else 0. The fifth
/// stack word is never read. Thiscall, five words, callee cleanup.
export!(thiscall, rw_0090a2e0(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, _a4: u32) -> u32 {
    unsafe {
        const LO_OFF: u32 = 0x04;
        const HI_OFF: u32 = 0x08;
        const CUR_OFF: u32 = 0x0C;
        const SAMPLE_ID: u32 = 1;
        let lo = ((this.wrapping_add(LO_OFF)) as *const i32).read_unaligned();
        let hi = ((this.wrapping_add(HI_OFF)) as *const i32).read_unaligned();
        let cur = ((this.wrapping_add(CUR_OFF)) as *const i32).read_unaligned();
        let in_range = cur >= lo && cur <= hi;
        let sampled = if in_range || lo < 0 {
            let _: u32 = callee_cdecl!(SAMPLE_ID, u32, a0, a1, a2, a3);
            1u32
        } else {
            0u32
        };
        ((this.wrapping_add(CUR_OFF)) as *mut u32)
            .write_unaligned((cur as u32).wrapping_add(1));
        sampled
    }
});
