// original: 0x00a09f50 mission_cleanup_entry_reset (proposed)
/// Reset a mission-cleanup entry selected by its second word.
///
/// Stdcall with two stack words; the first is unread. Loads the entry
/// pointer from the second and runs the entry initialiser on it (same
/// callee as the sibling reset paths). Returns the callee's answer.
lf_checker_rt::export!(stdcall, rw_00a09f50(_a1: u32, entry: u32) -> u32 {
    unsafe {
        const INIT: u32 = 0;
        lf_checker_rt::callee_thiscall!(INIT, u32, entry)
    }
});
