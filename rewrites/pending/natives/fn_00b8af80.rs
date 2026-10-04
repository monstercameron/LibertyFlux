// original: 0x00b8af80 ADD_TEXT_WIDGET
/// Script native `ADD_TEXT_WIDGET` (hash 0x7537050D).
///
/// Forwards one script argument (a text-slot handle) to the engine and stores the full 32-bit answer into the return slot.
export!(cdecl, rw_00b8af80(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
