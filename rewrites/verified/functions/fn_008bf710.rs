// original: 0x008BF710 ui_device_teardown (proposed)

/// Release the input device if the readiness check says it is live.
///
/// Calls the readiness callee with the global device handle; when its low
/// byte is zero nothing more happens and that answer is returned. Otherwise
/// the release callee runs with (handle, 1), the global handle is cleared,
/// and the detach callee runs as a thiscall on the fixed manager object with
/// argument 0, whose answer is returned (cdecl, no stack words).
lf_checker_rt::export!(cdecl, rw_008BF710() -> u32 {
    unsafe {
        /// Global input-device handle, cleared after release.
        const DEVICE_HANDLE: u32 = 0x01160C4C;
        /// Fixed manager object for the detach call.
        const MANAGER: u32 = 0x011A32C0;
        /// Callee ids in the contract: readiness, release, detach.
        const READY: u32 = 1;
        const RELEASE: u32 = 2;
        const DETACH: u32 = 3;
        let handle = lf_checker_rt::global::<u32>(DEVICE_HANDLE).read_unaligned();
        let answer = lf_checker_rt::callee_cdecl!(READY, u32, handle);
        if (answer & 0xFF) == 0 {
            return answer;
        }
        lf_checker_rt::callee_cdecl!(RELEASE, u32, handle, 1u32);
        lf_checker_rt::global::<u32>(DEVICE_HANDLE).write_unaligned(0);
        lf_checker_rt::callee_thiscall!(DETACH, u32, lf_checker_rt::relocated(MANAGER), 0u32)
    }
});
