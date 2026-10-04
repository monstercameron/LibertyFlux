// original: 0x00bb8d00 HAS_OVERRIDEN_SIT_IDLE_ANIM_FINISHED
/// Script native `HAS_OVERRIDEN_SIT_IDLE_ANIM_FINISHED` (hash 0x520A745D).
///
/// Forwards one script argument (a character handle) to the engine and stores
/// the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bb8d00(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
