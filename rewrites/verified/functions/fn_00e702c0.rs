// original: 0x00e702c0 delete_critsec_1a02268
/// Release the critical section at file VA 0x01A02268.
///
/// Forwards the constant critical-section pointer to the imported
/// `DeleteCriticalSection` and returns its answer, matching the value the
/// original leaves in EAX.
export!(cdecl, rw_00e702c0() -> u32 {
    unsafe {
        const CRIT: u32 = 0x01A02268;
        lf_checker_rt::callee_stdcall!(1, u32, relocated(CRIT))
    }
});
