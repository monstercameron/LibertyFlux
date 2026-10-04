// original: 0x00b98340 GET_MOUSE_WHEEL
/// Script native `GET_MOUSE_WHEEL` (hash 0x51870C68).
///
/// Forwards one script argument (a controller index) to the engine.
/// No return slot is written.
export!(cdecl, rw_00b98340(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
