// original: 0x00b93cd0 ACOS
/// Script native `ACOS` (hash 0x2E746E53).
///
/// Computes the arccosine of one float script argument.
///
/// /// Forwards the argument's bit pattern to the engine math routine
/// /// (which answers on the x87 register stack) and stores the returned
/// /// float into the return slot. Floats travel as raw bits, so the
/// /// forward and the store are bit-exact.
export!(cdecl, rw_00b93cd0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer: f32 = callee_cdecl!(1, f32, *args);
        let slot = *(ctx as *const u32) as *mut f32;
        *slot = answer;
        slot as u32
    }
});
