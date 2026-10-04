// original: 0x00a011c0 HAS_OBJECT_BEEN_PHOTOGRAPHED
/// Script native `HAS_OBJECT_BEEN_PHOTOGRAPHED` (hash 0x57895F38).
///
/// Forwards one script argument (an object handle) to the engine and
/// stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00a011c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
