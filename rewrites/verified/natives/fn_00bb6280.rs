// original: 0x00bb6280 CAN_THE_STAT_HAVE_STRING
/// Script native `CAN_THE_STAT_HAVE_STRING` (hash 0x0B651AFB).
///
/// Forwards one script argument (a stat id) to the engine and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bb6280(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
