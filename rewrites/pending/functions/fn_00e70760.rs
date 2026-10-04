// original: 0x00e70760 delete_critsec_1b432f8
/// Release the critical section at file VA 0x01B432F8.
///
/// Forwards the constant critical-section pointer to the imported
/// `DeleteCriticalSection` and returns its answer, matching the value the
/// original leaves in EAX.
export!(cdecl, rw_00e70760() -> u32 {
    unsafe {
        const CRIT: u32 = 0x01B432F8;
        lf_checker_rt::callee_stdcall!(1, u32, relocated(CRIT))
    }
});
