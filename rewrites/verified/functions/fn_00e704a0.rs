// original: 0x00e704a0 delete_critsec_1a05834
/// Release the critical section at file VA 0x01A05834.
///
/// Forwards the constant critical-section pointer to the imported
/// `DeleteCriticalSection` and returns its answer, matching the value the
/// original leaves in EAX.
export!(cdecl, rw_00e704a0() -> u32 {
    unsafe {
        const CRIT: u32 = 0x01A05834;
        lf_checker_rt::callee_stdcall!(1, u32, relocated(CRIT))
    }
});
