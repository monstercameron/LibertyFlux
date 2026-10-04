// original: 0x00bc52a0 CHANGE_GARAGE_TYPE
/// Script native `CHANGE_GARAGE_TYPE` (hash 0x6E0A438A).
///
/// Forwards two script arguments (a garage handle and the new type) to the
/// engine. No return slot is written.
export!(cdecl, rw_00bc52a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
