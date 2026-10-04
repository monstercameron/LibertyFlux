// original: 0x00bb8e70 REMOVE_COVER_POINT
/// Forward the cover handle to the cover engine function.
export!(cdecl, rw_00bb8e70(ctx: u32) -> u32 {
    const ARG_ARRAY: usize = 2;
    let c = ctx as *const u32;
    let args = unsafe { *c.add(ARG_ARRAY) } as *const u32;
    let cover = unsafe { *args.add(0) };
    callee_cdecl!(1, u32, cover)
});
