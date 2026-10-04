// original: 0x00b9ed90 GET_CHAR_DRAWABLE_VARIATION
/// Script native handler `GET_CHAR_DRAWABLE_VARIATION`.
///
/// Forwards the character handle and drawable slot, and stores the engine's full 32-bit answer in the script return slot.
lf_rn94_rt::export!(cdecl, rw_fn_00b9ed90(ctx: u32) -> u32 {
    unsafe {
        let args = *((ctx + 8) as *const u32);
        let a0 = *((args + 0) as *const u32);
        let a1 = *((args + 4) as *const u32);
        let answer = lf_rn94_rt::callee_cdecl!(1, u32, a0, a1);
        let slot = *(ctx as *const u32);
        *((slot) as *mut u32) = answer;
        answer
    }
});
