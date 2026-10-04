// original: 0x00bc5960 GET_CAR_BLOCKING_CAR
/// Script native `GET_CAR_BLOCKING_CAR` (hash 0x66B43B06).
///
/// Forwards 2 script argument(s) to the engine: 2 integer(s).
/// No return slot is written.
export!(cdecl, rw_00bc5960(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
