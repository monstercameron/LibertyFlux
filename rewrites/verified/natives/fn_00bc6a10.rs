// original: 0x00bc6a10 IS_CAR_STOPPED
/// Script native `IS_CAR_STOPPED` (hash 0x4A000F52).
///
/// Forwards one script argument (a vehicle handle) to the engine and
/// stores the low byte of its answer (zero-extended) into the
/// return slot.
export!(cdecl, rw_00bc6a10(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
