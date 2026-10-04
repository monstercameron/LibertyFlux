// original: 0x00BA00D0 IS_PED_DOING_DRIVEBY
// Rewrite of the IS_PED_DOING_DRIVEBY native handler.

/// Script native `IS_PED_DOING_DRIVEBY(ped)`.
///
/// Forwards the ped handle to the engine drive-by check and stores the
/// zero-extended low byte of its answer in the context's return slot.
export!(cdecl, rw_ba00d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        let answer: u32 = callee_cdecl!(1, u32, *args);
        let ret_slot = *(ctx as *const *mut u32);
        *ret_slot = answer & 0xFF;
        0
    }
});
