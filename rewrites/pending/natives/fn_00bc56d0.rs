// original: 0x00bc56d0 DONT_SUPPRESS_CAR_MODEL
/// Script native `DONT_SUPPRESS_CAR_MODEL` (hash 0x0348074B).
///
/// Forwards one script argument (a vehicle model hash) to the engine. No
/// return slot is written.
export!(cdecl, rw_00bc56d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
