// original: 0x00b94910 IS_EPISODE_AVAILABLE
/// Script native `IS_EPISODE_AVAILABLE` (hash 0x232800BD).
///
/// Forwards one integer script argument (an episode index) to the
/// engine and stores the low byte of its answer (zero-extended) into
/// the return slot.
export!(cdecl, rw_00b94910(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
