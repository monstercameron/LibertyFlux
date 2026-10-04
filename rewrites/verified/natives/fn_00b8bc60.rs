// original: 0x00b8bc60 CAN_RENDER_RADIOHUD_SPRITE_IN_MOBILE_PHONE
/// Script native `CAN_RENDER_RADIOHUD_SPRITE_IN_MOBILE_PHONE`
/// (hash 0x58F209BD).
///
/// Takes no script arguments: calls the engine worker with no arguments and
/// stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b8bc60(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
