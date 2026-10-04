// original: 0x009cc5f0 SET_PED_MOBILE_RING_TYPE
/// Script native handler `SET_PED_MOBILE_RING_TYPE`.
///
/// Forwards the ped handle and ring type to the audio engine; no script return value.
lf_rn94_rt::export!(cdecl, rw_fn_009cc5f0(ctx: u32) -> u32 {
    unsafe {
        let args = *((ctx + 8) as *const u32);
        let a0 = *((args + 0) as *const u32);
        let a1 = *((args + 4) as *const u32);
        let answer = lf_rn94_rt::callee_cdecl!(1, u32, a0, a1);
        answer
    }
});
