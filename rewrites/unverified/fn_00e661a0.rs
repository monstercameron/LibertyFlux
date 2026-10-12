// original: 0x00E661A0 initialize_radio_audio_entity (proposed)

/// Construct one audio entity object and register its teardown at exit.
///
/// Calls the entity constructor (thiscall, object pointer `0x012845d0`
/// in ECX, no stack arguments) and then the C run-time exit registration
/// (cdecl, one stack word: the teardown routine `0x00e71e70`). The
/// pushed word is popped back into ECX after the second call. The value
/// left in EAX is the registration callee's answer.
///
/// Original: 0x00E661A0 (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_00E661A0() -> u32 {
    unsafe {
        const OBJ: u32 = 0x012845d0;
        const HOOK: u32 = 0x00e71e70;
        const CTOR_CALLEE: u32 = 1;
        const ATEXIT_CALLEE: u32 = 2;
        let _: u32 =
            lf_checker_rt::callee_thiscall!(CTOR_CALLEE, u32, lf_checker_rt::relocated(OBJ));
        lf_checker_rt::callee_cdecl!(ATEXIT_CALLEE, u32, lf_checker_rt::relocated(HOOK))
    }
});
