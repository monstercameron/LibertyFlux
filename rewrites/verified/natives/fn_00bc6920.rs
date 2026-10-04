// original: 0x00bc6920 IS_CAR_LOW_RIDER
/// Script native `IS_CAR_LOW_RIDER` (hash 0x6B3D5D45).
///
/// Forwards one script argument (a vehicle handle) to the engine and stores
/// the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bc6920(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
