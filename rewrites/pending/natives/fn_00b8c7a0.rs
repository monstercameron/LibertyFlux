// original: 0x00b8c7a0 GET_BLIP_INFO_ID_ROTATION
/// Script native `GET_BLIP_INFO_ID_ROTATION` (hash 0x6FBA4274).
///
/// Forwards one script argument (a blip handle) to the engine and stores its full 32-bit answer into the return slot.
///
/// Unlike the boolean natives, this handler keeps the whole 32-bit answer (`mov`, not `movzx`).
export!(cdecl, rw_00b8c7a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
