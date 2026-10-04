// original: 0x00b8d240 PRINT_WITH_4_NUMBERS_NOW
/// Script native handler `PRINT_WITH_4_NUMBERS_NOW`.
///
/// Forwards the text label, duration and five parameters to the text queue; no script return value.
lf_rn94_rt::export!(cdecl, rw_fn_00b8d240(ctx: u32) -> u32 {
    unsafe {
        let args = *((ctx + 8) as *const u32);
        let a0 = *((args + 0) as *const u32);
        let a1 = *((args + 4) as *const u32);
        let a2 = *((args + 8) as *const u32);
        let a3 = *((args + 12) as *const u32);
        let a4 = *((args + 16) as *const u32);
        let a5 = *((args + 20) as *const u32);
        let a6 = *((args + 24) as *const u32);
        let answer = lf_rn94_rt::callee_cdecl!(1, u32, a0, a1, a2, a3, a4, a5, a6);
        answer
    }
});
