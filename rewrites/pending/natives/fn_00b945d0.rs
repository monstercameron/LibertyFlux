// original: 0x00b945d0 GET_LINE_HEIGHT
/// Script native `GET_LINE_HEIGHT` (hash 0x150B0C33).
///
/// Takes no script arguments: calls the engine worker with no arguments,
/// which answers with a float in ST0, and stores the float's bits into the
/// return slot.
export!(cdecl, rw_00b945d0(ctx: *const u8) -> u32 {
    unsafe {
        let height: f32 = callee_cdecl!(1, f32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = height.to_bits();
        slot as u32
    }
});
