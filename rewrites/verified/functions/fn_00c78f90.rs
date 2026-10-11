// original: 0x00C78F90 streaming_shutdown_tail_entry
/// Forward the streaming shutdown entry to its shared implementation.
///
/// This entry is a tail jump with no local state changes. The `this` pointer
/// is forwarded in ECX, and the implementation's EAX result is returned to
/// the caller. The original uses the 32-bit thiscall convention and has no
/// explicit stack arguments.
///
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00c78f90(this_pointer: u32) -> u32 {
    const SHUTDOWN_CALLEE: u32 = 1;
    unsafe { lf_checker_rt::callee_thiscall!(SHUTDOWN_CALLEE, u32, this_pointer) }
}
