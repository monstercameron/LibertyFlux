// original: 0x00bc5d80 GET_CURRENT_BASIC_POLICE_CAR_MODEL
/// Script native `GET_CURRENT_BASIC_POLICE_CAR_MODEL`.
///
/// Forwards one script argument to the engine. No return slot is written.
export!(cdecl, rw_00bc5d80(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});

