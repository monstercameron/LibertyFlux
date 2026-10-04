// original: 0x00bd8b50 REGISTER_MOD
/// Script native `REGISTER_MOD` (hash 0x327866F6).
///
/// Forwards one script argument (a mod slot index) to the engine and stores
/// its full 32-bit answer into the return slot.
export!(cdecl, rw_00bd8b50(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
