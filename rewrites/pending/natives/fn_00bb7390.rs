// original: 0x00bb7390 HAS_COLLISION_FOR_MODEL_LOADED
/// Script native `HAS_COLLISION_FOR_MODEL_LOADED` (hash 0x7C3939E7).
///
/// Forwards one script argument (a model hash) to the engine and stores
/// the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bb7390(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer: u32 = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
