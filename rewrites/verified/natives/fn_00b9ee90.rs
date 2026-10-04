// original: 0x00b9ee90 GET_CHAR_HIGHEST_PRIORITY_EVENT
/// Script native `GET_CHAR_HIGHEST_PRIORITY_EVENT` (hash 0x061A75D3).
///
/// Forwards two script arguments to the engine. No return slot is written.
export!(cdecl, rw_00b9ee90(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
