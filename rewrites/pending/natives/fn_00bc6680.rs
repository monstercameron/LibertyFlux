// original: 0x00bc6680 IS_CAR_DEAD
/// Script native `IS_CAR_DEAD` (hash 0x2AAB340A).
///
/// Forwards one script argument (a vehicle handle) to the engine and
/// stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bc6680(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
