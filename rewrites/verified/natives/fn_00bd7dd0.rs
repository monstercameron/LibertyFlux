// original: 0x00bd7dd0 IS_DAMAGE_TRACKER_ACTIVE_ON_NETWORK_ID
/// Script native `IS_DAMAGE_TRACKER_ACTIVE_ON_NETWORK_ID` (hash 0x5A2F2DD1).
///
/// Forwards one script argument (a network id) to the engine and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bd7dd0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
