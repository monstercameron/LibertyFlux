// original: 0x00b94bb0 OBFUSCATE_STRING
/// Script native `OBFUSCATE_STRING` (hash 0x04F12617).
///
/// Forwards one script argument (a string pointer) to the engine's string
/// obfuscator and stores its full 32-bit answer (the static-buffer pointer)
/// into the return slot.
export!(cdecl, rw_00b94bb0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
