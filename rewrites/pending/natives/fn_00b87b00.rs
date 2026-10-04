// original: 0x00b87b00 SET_HINT_FOV
/// Script native `SET_HINT_FOV` (hash 0x2F9751E2).
///
/// Forwards one script argument (a float bit-pattern) to the engine. No
/// return slot is written.
export!(cdecl, rw_00b87b00(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
