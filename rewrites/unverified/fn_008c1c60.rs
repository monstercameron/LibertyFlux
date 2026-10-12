// original: 0x008C1C60 audio_engine_init_and_start

/// Initialise the audio engine, then start it unless already running.
///
/// Calls the engine initialiser, then reads the running word `STATE: when
/// it is zero tail-calls the engine starter with the engine object and
/// returns its result, otherwise returns the initialiser's result. Cdecl,
/// no arguments.
lf_checker_rt::export!(cdecl, rw_008C1C60() -> u32 {
    unsafe {
        const STATE: u32 = 0x011D_6FD0;
        const ENGINE: u32 = 0x0117_6888;
        let first = lf_checker_rt::callee_cdecl!(1, u32);
        let state = (lf_checker_rt::relocated(STATE) as *const u32).read_unaligned();
        if state == 0 {
            lf_checker_rt::callee_thiscall!(2, u32, lf_checker_rt::relocated(ENGINE))
        } else {
            first
        }
    }
});
