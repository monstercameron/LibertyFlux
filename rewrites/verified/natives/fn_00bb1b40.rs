// original: 0x00bb1b40 AWARD_ACHIEVEMENT
/// Script native `AWARD_ACHIEVEMENT` (hash 0x5ED03255).
///
/// Forwards one script argument (an achievement id) to the engine and stores
/// the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bb1b40(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
