// original: 0x008C7C40 stream_init_guarded
/// Initialise the streaming backend when the ready check passes.
///
/// Asks the ready callee; on 0 returns at once. Otherwise clears the
/// pending flag, runs the setup callee with mode 0, then dispatches the
/// pending handle global through the handler callee with the handler
/// token. Returns nothing. Original: cdecl, no stack words.
lf_checker_rt::export!(cdecl, rw_008c7c40() -> u32 {
    unsafe {
        const READY_CALLEE: u32 = 1;
        const SETUP_CALLEE: u32 = 2;
        const HANDLE_CALLEE: u32 = 3;
        const HANDLER_TOKEN: u32 = 0x018B7A4D;
        let ready: u32 = lf_checker_rt::callee_cdecl!(READY_CALLEE, u32);
        if ready == 0 {
            return 0;
        }
        *lf_checker_rt::global::<u8>(0x1172cd3) = 0;
        lf_checker_rt::callee_cdecl!(SETUP_CALLEE, u32, 0);
        let handle = *lf_checker_rt::global::<u32>(0x1172cd4);
        lf_checker_rt::callee_thiscall!(HANDLE_CALLEE, u32, HANDLER_TOKEN,
                                        handle);
        0
    }
});
