// original: 0x00bc55a0 DELETE_CAR
/// Script native `DELETE_CAR` (hash 0x7F71342D).
///
/// Forwards one script argument (a vehicle handle) to the engine. No return
/// slot is written.
export!(cdecl, rw_00bc55a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
