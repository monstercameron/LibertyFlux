// original: 0x00b98780 USING_STANDARD_CONTROLS
/// Script native `USING_STANDARD_CONTROLS` (hash 0x5F4571E5).
///
/// Script arguments: none.
///
/// Forwards no arguments to the engine routine.
/// The engine's byte answer is zero-extended into the return slot.
export!(cdecl, rw_00b98780(ctx: *const u8) -> u32 {
    unsafe {
        let slot = *(ctx as *const *mut u32);
        let answer: u32 = callee_cdecl!(1, u32, );
        *slot = answer & 0xFF;
        slot as u32
    }
});
