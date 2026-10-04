// original: 0x00b94570 GET_INTERIOR_HEADING
/// Script native `GET_INTERIOR_HEADING` (hash 0x73245AB3).
///
/// Forwards two script arguments (an interior handle and an out-pointer)
/// to the engine. No return slot is written by the handler itself.
export!(cdecl, rw_00b94570(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
