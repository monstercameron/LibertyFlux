// original: 0x00ba2b70 WAS_PED_SKELETON_UPDATED
/// Script native `WAS_PED_SKELETON_UPDATED` (hash 0x3E8443E0).
///
/// Forwards one script argument (a character handle) to the engine.
/// Stores the low byte of the engine answer (zero-extended) into the return slot.
export!(cdecl, rw_00ba2b70(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
