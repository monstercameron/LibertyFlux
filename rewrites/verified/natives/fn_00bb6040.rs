// original: 0x00bb6040 REQUEST_SCRIPT
/// Script native `REQUEST_SCRIPT` (hash 0x6FFE0DFD).
///
/// Forwards 1 script argument(s) to the engine: 1 integer(s).
/// No return slot is written.
export!(cdecl, rw_00bb6040(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
