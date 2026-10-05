// original: 0x00942860 streaming_config_init (proposed)

/// Register the three streaming configuration variables and reset state.
///
/// Calls the config registrar (cdecl, two arguments: variable address and
/// size) for the two one-byte flags and one four-byte word, clears the
/// state word, and returns 1.
///
/// Original: 0x00942860 (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_00942860() -> u32 {
    unsafe {
        const FLAG_A: u32 = 0x01037598;
        const FLAG_B: u32 = 0x01037599;
        const WORD_C: u32 = 0x012B6250;
        const STATE: u32 = 0x011D6FA0;
        const CALLEE: u32 = 1;
        let _: u32 = lf_checker_rt::callee_cdecl!(
            CALLEE,
            u32,
            lf_checker_rt::relocated(FLAG_A),
            1u32
        );
        let _: u32 = lf_checker_rt::callee_cdecl!(
            CALLEE,
            u32,
            lf_checker_rt::relocated(FLAG_B),
            1u32
        );
        lf_checker_rt::global::<u32>(STATE).write(0);
        let _: u32 = lf_checker_rt::callee_cdecl!(
            CALLEE,
            u32,
            lf_checker_rt::relocated(WORD_C),
            4u32
        );
        1
    }
});
