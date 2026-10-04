// original: 0x00b94740 GET_STRING_WIDTH_WITH_NUMBER
/// Script native handler `GET_STRING_WIDTH_WITH_NUMBER`.
///
/// Forwards the text label and number to the text engine, which answers with a float width, and stores it in the script return slot.
lf_rn94_rt::export!(cdecl, rw_fn_00b94740(ctx: u32) -> u32 {
    unsafe {
        let args = *((ctx + 8) as *const u32);
        let a0 = *(args as *const u32);
        let a1 = *((args + 4) as *const u32);
        let width: f32 = lf_rn94_rt::callee_cdecl!(1, f32, a0, a1);
        let slot = *(ctx as *const u32);
        *((slot) as *mut f32) = width;
        slot
    }
});
