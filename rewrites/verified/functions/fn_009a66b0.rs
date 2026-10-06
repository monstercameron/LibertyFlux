// original: 0x009A66B0 MOBILE_PRERING

/// Registers the mobile-prering callbacks with the audio manager.
///
/// No arguments. Calls the single-shot registrar (callee 1, thiscall on
/// the manager at `MANAGER`, one word: the handler at 0x009A5880), then
/// the named registrar (callee 2, thiscall on the same manager, two words:
/// the tag string at `TAG` first and the callback at 0x009A6800 second).
/// Returns the second registrar's answer.
lf_checker_rt::export!(cdecl, rw_009a66b0() -> u32 {
    unsafe {
        const REGISTRAR_ONE: u32 = 1;
        const REGISTRAR_NAMED: u32 = 2;
        const MANAGER: u32 = 0x001288560;
        const HANDLER: u32 = 0x009A5880;
        const CALLBACK: u32 = 0x009A6800;
        const TAG: u32 = 0x00E91A34;
        let manager = lf_checker_rt::relocated(MANAGER);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            REGISTRAR_ONE,
            u32,
            manager,
            lf_checker_rt::relocated(HANDLER)
        );
        lf_checker_rt::callee_thiscall!(
            REGISTRAR_NAMED,
            u32,
            manager,
            lf_checker_rt::relocated(TAG),
            lf_checker_rt::relocated(CALLBACK)
        )
    }
});
