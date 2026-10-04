// original: 0x00ba0130 IS_PED_IN_COMBAT
/// Script native `IS_PED_IN_COMBAT` (hash 0x020106D6).
///
/// Forwards one script argument to the engine and stores the low byte of
/// its answer (zero-extended) into the return slot.
export!(cdecl, rw_00ba0130(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
