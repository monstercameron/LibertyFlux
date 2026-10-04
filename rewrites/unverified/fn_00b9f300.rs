// original: 0x00b9f300 GET_INTERIOR_FROM_DUMMY_CHAR
/// Script native `GET_INTERIOR_FROM_DUMMY_CHAR` (hash 0x380751A9).
///
/// Forwards a character handle and an out-pointer to the engine.
/// No return slot is written.
export!(cdecl, rw_00b9f300(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1),)
    }
});
