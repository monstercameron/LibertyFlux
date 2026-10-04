// original: 0x00b982e0 GET_MOUSE_INPUT
/// Script native `GET_MOUSE_INPUT` (hash 0x447B154B).
///
/// Forwards 2 script argument(s) to the engine: 2 integer(s).
/// No return slot is written.
export!(cdecl, rw_00b982e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
