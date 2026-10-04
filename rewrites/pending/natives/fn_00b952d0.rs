// original: 0x00b952d0 UNOBFUSCATE_STRING
/// Script native `UNOBFUSCATE_STRING` (hash 0x2186777E).
///
/// Forwards one script argument (an obfuscated string pointer) to the engine and stores its full 32-bit answer into the return slot.
///
/// Unlike the boolean natives, this handler keeps the whole 32-bit answer (`mov`, not `movzx`).
export!(cdecl, rw_00b952d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
