// original: 0x00bc6580 HAS_CAR_BEEN_DROPPED_OFF
/// Script native `HAS_CAR_BEEN_DROPPED_OFF` (hash 0x024C3A6C).
///
/// Forwards one script argument to the engine and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bc6580(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
