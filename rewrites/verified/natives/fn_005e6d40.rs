// original: 0x005e6d40 CREATE_MOBILE_PHONE
/// Script native handler `CREATE_MOBILE_PHONE`.
///
/// Forwards the phone handle to the phone engine; no script return value.
lf_rn94_rt::export!(cdecl, rw_fn_005e6d40(ctx: u32) -> u32 {
    unsafe {
        let args = *((ctx + 8) as *const u32);
        let a0 = *((args + 0) as *const u32);
        let answer = lf_rn94_rt::callee_cdecl!(1, u32, a0);
        answer
    }
});
