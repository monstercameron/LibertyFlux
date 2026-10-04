// original: 0x005e73e0 INIT_FRONTEND_HELPER_TEXT
/// Script native `INIT_FRONTEND_HELPER_TEXT` (hash 0x617B191D).
///
/// Writes zero to the engine's frontend-helper-text state word. No engine
/// call is made, the context pointer is not read, and no return slot is
/// written. The exit register is caller garbage on this path, so the
/// contract does not compare it; the returned zero is arbitrary.
export!(cdecl, rw_005e73e0(ctx: *const u8) -> u32 {
    unsafe {
        *global::<u32>(0x11609e8) = 0;
        let _ = ctx;
        0
    }
});
