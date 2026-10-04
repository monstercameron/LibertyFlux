// original: 0x00bb63e0 GET_STAT_FRONTEND_VISIBILITY
/// Script native `GET_STAT_FRONTEND_VISIBILITY` (hash 0x38905687).
///
/// Reports whether a statistic is visible in the frontend. Forwards one
/// script argument (the stat id) to the engine and stores the low byte of
/// its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bb63e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
