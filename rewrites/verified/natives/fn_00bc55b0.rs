// original: 0x00bc55b0 DELETE_CAR_GENERATOR
/// Script native `DELETE_CAR_GENERATOR` (hash 0x76E738A3).
///
/// Forwards 1 script argument(s) to the engine: 1 integer(s).
/// No return slot is written.
export!(cdecl, rw_00bc55b0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
