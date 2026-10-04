// original: 0x00b8bc40 CAN_FONT_BE_LOADED
/// Script native handler `CAN_FONT_BE_LOADED` (hash 0x1E2A5820).
///
/// Forwards script argument 0 to the engine worker and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b8bc40(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        let answer: u32 = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
