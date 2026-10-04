// original: 0x00bd4020 GET_SCREEN_RESOLUTION
//
// Script native handler: forwards two out-pointers to one engine function
// (cdecl/2), which reports the resolution through them. The handler itself
// stores nothing to the return slot.
export!(cdecl, rw_00bd4020(ctx: *const u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *const *const u32);
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
