// original: 0x008C6E20 stream_forward_guarded
/// Forward to the request callee only in the matching request state.
///
/// When `want` is nonzero the state global must read 1 and the mode bit
/// forwarded is 1, otherwise the state must read 2 and the mode bit is 0;
/// on a mismatch the result is 0 without calling out. The request block
/// address always accompanies the call. Returns the callee's answer, or 0
/// with stale upper bits on a mismatch; only the low byte is behaviour.
/// Original: cdecl, one stack word.
lf_checker_rt::export!(cdecl, rw_008c6e20(want: u32) -> u32 {
    unsafe {
        const REQ_CALLEE: u32 = 1;
        const REQ_BASE_FILE_VA: u32 = 0x1172FE0;
        const STATE_OFF: u32 = 0x11730F8;
        let base = lf_checker_rt::relocated(REQ_BASE_FILE_VA);
        let state = *lf_checker_rt::global::<u32>(STATE_OFF);
        if (want as u8) != 0 {
            if state != 1 {
                return 0;
            }
            lf_checker_rt::callee_cdecl!(REQ_CALLEE, u32, 1, base)
        } else {
            if state != 2 {
                return 0;
            }
            lf_checker_rt::callee_cdecl!(REQ_CALLEE, u32, 0, base)
        }
    }
});
