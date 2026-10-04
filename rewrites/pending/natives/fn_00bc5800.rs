// original: 0x00bc5800 FIND_TRAIN_DIRECTION
/// Script native `FIND_TRAIN_DIRECTION` (hash 0x013C1EB7).
///
/// Forwards one script argument (a vehicle handle) to the engine and stores
/// the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bc5800(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
