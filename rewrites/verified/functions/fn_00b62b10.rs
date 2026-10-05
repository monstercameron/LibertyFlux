// original: 0x00B62B10 veh_copy3_forward
/// Copy 3 dwords from `[a1+0x30]` to `a0`, resolve twice, forward, copy result.
///
/// Copies `[a1+0x30..0x38]` to `[a0..a0+8]`. Resolves the context twice
/// (stubbed, thiscall/0); returns `a0` when the first resolution is null.
/// Otherwise reads a select word at `[ctx2 + a2*4 + 0x64]` and calls the
/// converter (stubbed, thiscall/3) with `(frame_scratch, a1, word)`, then copies
/// the 4 dwords at the returned pointer over `[a0..a0+12]` and returns `a0`.
/// The scratch pointer lives in the function's own frame, so the call argument
/// is skipped in the comparison. Thiscall, three stack words.
export!(thiscall, rw_00b62b10(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const DISP: u32 = 0x64;
        for (s, d) in [(0x30u32, 0u32), (0x34, 4), (0x38, 8)] {
            ((a0 + d) as *mut u32).write_unaligned(((a1 + s) as *const u32).read_unaligned());
        }
        let h1: u32 = callee_thiscall!(1, u32, this);
        if h1 == 0 {
            return a0;
        }
        let h2: u32 = callee_thiscall!(1, u32, this);
        let w = ((h2.wrapping_add(a2.wrapping_mul(4)).wrapping_add(DISP)) as *const u32)
            .read_unaligned();
        let mut scratch = [0u32; 8];
        let p: u32 = callee_thiscall!(2, u32, this, scratch.as_mut_ptr() as u32, a1, w);
        for o in [0u32, 4, 8, 12] {
            ((a0 + o) as *mut u32).write_unaligned(((p + o) as *const u32).read_unaligned());
        }
        a0
    }
});
