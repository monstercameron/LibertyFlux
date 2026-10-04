// original: 0x00bc5320 CLOSE_ALL_CAR_DOORS
/// Script native `CLOSE_ALL_CAR_DOORS` (hash 0x56B8674F).
///
/// Forwards one script argument (a vehicle handle) to the engine. No
/// return slot is written.
export!(cdecl, rw_00bc5320(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
