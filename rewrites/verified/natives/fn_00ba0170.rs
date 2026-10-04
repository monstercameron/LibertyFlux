// original: 0x00ba0170 IS_PED_IN_GROUP
/// Script native `IS_PED_IN_GROUP` (hash 0x365054A7).
///
/// Forwards one script argument (a character handle) to the engine and
/// stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00ba0170(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
