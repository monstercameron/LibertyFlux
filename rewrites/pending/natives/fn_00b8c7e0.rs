// original: 0x00b8c7e0 GET_BLIP_SPRITE
/// Script native `GET_BLIP_SPRITE` (hash 0x30B1316B).
///
/// Forwards one script argument (a blip handle) to the engine and stores
/// the engine's full 32-bit answer (the sprite id) into the return slot.
export!(cdecl, rw_00b8c7e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
