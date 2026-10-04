// original: 0x00bc52c0 CHANGE_PLAYBACK_TO_USE_AI
/// Script native handler `CHANGE_PLAYBACK_TO_USE_AI`.
///
/// Forwards the recording handle to the playback engine; no script return value.
lf_rn94_rt::export!(cdecl, rw_fn_00bc52c0(ctx: u32) -> u32 {
    unsafe {
        let args = *((ctx + 8) as *const u32);
        let a0 = *((args + 0) as *const u32);
        let answer = lf_rn94_rt::callee_cdecl!(1, u32, a0);
        answer
    }
});
