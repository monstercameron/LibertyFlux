// original: 0x00bb8de0 MODIFY_CHAR_MOVE_BLEND_RATIO
/// Forward (char, ratio-bits) to the move-blend engine function.
rt::export!(cdecl, rw_00bb8de0(ctx: u32) -> u32 {
    const ARG_ARRAY: usize = 2;
    let c = ctx as *const u32;
    let args = unsafe { *c.add(ARG_ARRAY) } as *const u32;
    let ch = unsafe { *args.add(0) };
    let ratio_bits = unsafe { *args.add(1) };
    rt::callee_cdecl!(1, u32, ch, ratio_bits)
});
