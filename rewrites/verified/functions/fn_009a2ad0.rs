// original: 0x009A2AD0 audio_maybe_notify_and_release (proposed)

/// Conditionally notify, then release two owned handles.
///
/// When the global mode is not 1, the two global generation counters agree
/// and the global state is not 0x12, the notifier (callee 1, cdecl/2 of
/// `this`+8 and constant 1) runs. Then each of `this`+0x60 and `this`+0x64,
/// when nonzero, is passed with a zero word to the handle release (callee
/// 2, thiscall/1). The return is the last answer produced on the path: the
/// notifier's, a release's, the second counter's, or the entry `eax` when
/// the mode test fails first; the contract fixes entry `eax` to zero so the
/// whole return compares. Thiscall with no stack words.
lf_checker_rt::export!(thiscall, rw_009A2AD0(this: u32) -> u32 {
    unsafe {
        const MODE: u32 = 0x011F7060;
        const GEN_A: u32 = 0x012088B4;
        const GEN_B: u32 = 0x00F1C040;
        const STATE: u32 = 0x01037720;
        const READY: u32 = 1;
        const SKIP_STATE: u32 = 0x12;
        const PAYLOAD: u32 = 8;
        const HANDLE_A: u32 = 0x60;
        const HANDLE_B: u32 = 0x64;
        const NOTIFY_CALLEE: u32 = 1;
        const RELEASE_CALLEE: u32 = 2;
        let g = |va: u32| (lf_checker_rt::global::<u32>(va) as *const u32).read_unaligned();
        let mut ans: u32 = 0; // Entry eax, fixed to zero by the contract.
        if g(MODE) != READY {
            let a = g(GEN_A);
            ans = a;
            if a == g(GEN_B) && g(STATE) != SKIP_STATE {
                let payload =
                    ((this.wrapping_add(PAYLOAD)) as *const u32).read_unaligned();
                ans = lf_checker_rt::callee_cdecl!(NOTIFY_CALLEE, u32, payload, 1);
            }
        }
        let ha = ((this.wrapping_add(HANDLE_A)) as *const u32).read_unaligned();
        if ha != 0 {
            ans = lf_checker_rt::callee_thiscall!(RELEASE_CALLEE, u32, ha, 0);
        }
        let hb = ((this.wrapping_add(HANDLE_B)) as *const u32).read_unaligned();
        if hb != 0 {
            ans = lf_checker_rt::callee_thiscall!(RELEASE_CALLEE, u32, hb, 0);
        }
        ans
    }
});
