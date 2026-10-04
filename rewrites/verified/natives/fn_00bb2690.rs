// original: 0x00bb2690 IS_THIS_PED_A_PLAYER
/// Script native `IS_THIS_PED_A_PLAYER` (hash 0x37C85316).
///
/// Forwards one script argument (a ped handle) to the engine and stores the
/// low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bb2690(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
