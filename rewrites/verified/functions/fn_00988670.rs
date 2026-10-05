// original: 0x00988670 audCutscene_reset_forward (proposed)

/// Cutscene reset forwarder: runs the entity reset and always returns 1.
///
/// Calls the callee at slot id 1 with argument 1 and `this` fixed to the
/// shared object at `SHARED_ENTITY`, ignores the answer and returns 1 in `al`.
/// Original: stdcall, no stack words.
lf_checker_rt::export!(stdcall, rw_00988670() -> u32 {
    const SHARED_ENTITY: u32 = 0x1282fa8;
    const RESET_CALLEE: u32 = 1;
    unsafe {
        let _: u32 = lf_checker_rt::callee_thiscall!(
            RESET_CALLEE,
            u32,
            lf_checker_rt::relocated(SHARED_ENTITY),
            1
        );
    }
    1
});
