// original: 0x00bc6ab0 IS_CAR_STREET_RACER
/// Script native `IS_CAR_STREET_RACER` (hash 0x24DF32CC).
///
/// Forwards one script argument (a vehicle handle) to the engine and stores
/// the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bc6ab0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
