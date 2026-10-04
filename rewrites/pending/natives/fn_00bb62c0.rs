// original: 0x00BB62C0 CONVERT_METRES_TO_FEET_INT
// Rewrite of the CONVERT_METRES_TO_FEET_INT native handler.

/// Script native `CONVERT_METRES_TO_FEET_INT(metres)`.
///
/// Forwards the single argument to the engine conversion routine and stores
/// its full answer in the context's return slot.
export!(cdecl, rw_bb62c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        let answer: u32 = callee_cdecl!(1, u32, *args);
        let ret_slot = *(ctx as *const *mut u32);
        *ret_slot = answer;
        answer
    }
});
