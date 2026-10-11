// original: 0x00AE9060 release_mainloop_timer

/// Releases a non-null timer handle through the shared timer service. Null
/// handles are ignored. The service pointer is a relocated global object.
///
/// Calling convention: cdecl with one 32-bit handle argument.
lf_checker_rt::export!(cdecl, rw_00ae9060(handle: u32) -> () {
    if handle != 0 {
        const TIMER_SERVICE_VA: u32 = 0x015c_3ba8;
        lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(TIMER_SERVICE_VA), handle);
    }
});
