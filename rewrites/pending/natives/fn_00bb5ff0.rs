// original: 0x00bb5ff0 HAS_SCRIPT_LOADED
/// Script native `HAS_SCRIPT_LOADED` (hash 0x2A171915).
///
/// Forwards one script argument (a script handle) to the engine and stores
/// the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bb5ff0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
