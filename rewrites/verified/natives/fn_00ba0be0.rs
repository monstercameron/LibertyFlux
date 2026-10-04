// original: 0x00ba0be0 MP_GET_VARIATION_SETUP
/// Script native `MP_GET_VARIATION_SETUP` (hash 0x3775138E).
///
/// Forwards arg0, arg1, arg2, arg3, arg4 to the engine.
/// Writes the full engine answer into the return slot.
export!(cdecl, rw_00ba0be0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let ans = callee_cdecl!(1, u32, *args.add(0), *args.add(1), *args.add(2), *args.add(3), *args.add(4));
        let slot = (*(ctx as *const u32)) as *mut u32;
        *slot = ans;
        ans
    }
});
