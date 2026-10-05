// original: 0x00885070 stream_driver_shutdown (proposed)
/// Shut the streaming driver down: stop its four services, then sleep one second.
///
/// Calls the four service stop routines in order (intercepted callees 1-4,
/// no stack arguments; the fourth receives the driver object from the global
/// at file address `0x17f5e94` in `ecx`), then sleeps 1,000 ms through the
/// imported `Sleep` (intercepted callee 5, stdcall, one argument).
///
/// Original: cdecl, no stack arguments, no return value.
lf_checker_rt::export!(cdecl, rw_00885070() -> u32 {
    unsafe {
        const DRIVER_OBJECT: u32 = 0x017f_5e94;
        const SLEEP_MS: u32 = 1000;
        const STOP_A_CALLEE: u32 = 1;
        const STOP_B_CALLEE: u32 = 2;
        const STOP_C_CALLEE: u32 = 3;
        const STOP_D_CALLEE: u32 = 4;
        const SLEEP_CALLEE: u32 = 5;
        let _: u32 = lf_checker_rt::callee_cdecl!(STOP_A_CALLEE, u32,);
        let _: u32 = lf_checker_rt::callee_cdecl!(STOP_B_CALLEE, u32,);
        let _: u32 = lf_checker_rt::callee_cdecl!(STOP_C_CALLEE, u32,);
        let driver =
            (lf_checker_rt::global::<u32>(DRIVER_OBJECT) as *const u32).read_unaligned();
        let _: u32 = lf_checker_rt::callee_thiscall!(STOP_D_CALLEE, u32, driver);
        let _: u32 = lf_checker_rt::callee_stdcall!(SLEEP_CALLEE, u32, SLEEP_MS);
        0
    }
});
