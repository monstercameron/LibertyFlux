// original: 0x00C377A0 CHeli::vf77

/// Ask the vehicle cleanup predicate about this object and the three supplied
/// cleanup values. If its low-byte result is nonzero, call the attached-object
/// CCD synchronizer and return one; otherwise return zero. The first callee
/// receives the object in ECX and the three stack arguments in order. This
/// thiscall method takes three 32-bit stack arguments and returns its boolean
/// result in AL.
lf_checker_rt::export!(thiscall, rw_00c377a0(
    this: *mut u8,
    cleanup_radius: u32,
    cleanup_delay: u32,
    cleanup_mode: u32,
) -> u32 {
    const CLEANUP_PREDICATE: u32 = 1;
    const SYNC_ATTACHED_CCD: u32 = 2;

    let should_cleanup: u8 = lf_checker_rt::callee_thiscall!(
        CLEANUP_PREDICATE,
        u8,
        this as u32,
        cleanup_radius,
        cleanup_delay,
        cleanup_mode,
    );
    if should_cleanup != 0 {
        let _: u32 = lf_checker_rt::callee_thiscall!(SYNC_ATTACHED_CCD, u32, this as u32);
        1
    } else {
        0
    }
});
