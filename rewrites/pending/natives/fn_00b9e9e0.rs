// original: 0x00b9e9e0 DOES_GROUP_EXIST
/// Script native `DOES_GROUP_EXIST` (hash 0x3D385F6D).
///
/// Forwards one script argument (a group handle) to the engine and stores
/// the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b9e9e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
