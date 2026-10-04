// original: 0x00bd3fa0 GET_ASPECT_RATIO
/// Script native `GET_ASPECT_RATIO`.
///
/// Takes no script arguments: calls the engine worker, which returns a
/// float on the x87 stack, and stores it into the return slot.
/// Returns the slot pointer.
export!(cdecl, rw_00bd3fa0(ctx: *const u8) -> u32 {
    unsafe {
        let answer: f32 = callee_cdecl!(1, f32,);
        let slot = *(ctx as *const u32) as *mut f32;
        *slot = answer;
        slot as u32
    }
});
