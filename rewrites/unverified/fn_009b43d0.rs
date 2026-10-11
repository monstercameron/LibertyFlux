// original: 0x009B43D0 audPoliceScanner::shutdown

/// Runs the scanner update, sends the pending sound handle to the engine,
/// clears the two sound-state words, and tail-forwards to the base shutdown
/// routine. The base routine's return value is returned unchanged.
lf_checker_rt::export!(thiscall, rw_009b43d0(this: u32) -> u32 {
    const SOUND_HANDLE: u32 = 0x5c;
    const SOUND_STATE: u32 = 0x60;

    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
        let pending_sound = ((this + SOUND_HANDLE) as *const u32).read_unaligned();
        let _ = lf_checker_rt::callee_cdecl!(2, u32, pending_sound);
        ((this + SOUND_HANDLE) as *mut u32).write_unaligned(0);
        ((this + SOUND_STATE) as *mut u32).write_unaligned(0);
        lf_checker_rt::callee_thiscall!(3, u32, this)
    }
});
