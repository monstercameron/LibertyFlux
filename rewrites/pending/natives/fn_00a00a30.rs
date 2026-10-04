// original: 0x00a00a30 DOES_PICKUP_EXIST
/// Script native `DOES_PICKUP_EXIST` (hash 0x7B567F1A).
///
/// Forwards one script argument (a pickup handle) to the engine and stores
/// the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00a00a30(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
