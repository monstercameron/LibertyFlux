// original: 0x00a6ee30 CTaskComplexPlayerOnFoot::vf1

/// Clone hook of the on-foot player task: clones through the task manager,
/// then copies this task's mode byte onto the clone.
///
/// Fetches the current manager through the anchor at `MANAGER_ANCHOR`
/// (callee 1). With no manager the original still performs the copy with a
/// null destination, faulting on the write to `MODE` bytes past null; the
/// rewrite does the same, so the fault matches. Otherwise the clone routine
/// (callee 2) runs with the manager in ecx, the mode byte at `this + MODE`
/// is copied to `clone + MODE`, and the clone is returned.
///
/// Original: thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00a6ee30(this: u32) -> u32 {
    unsafe {
        const MANAGER_ANCHOR: u32 = 0x0167_e2a0;
        const MODE: u32 = 0x50;
        const GET_MANAGER: u32 = 1;
        const CLONE_INTO: u32 = 2;

        let anchor = (lf_checker_rt::relocated(MANAGER_ANCHOR) as *const u32).read_unaligned();
        let mgr: u32 = lf_checker_rt::callee_thiscall!(GET_MANAGER, u32, anchor);
        let mode = ((this as *const u8).wrapping_byte_offset(MODE as isize)).read();
        if mgr == 0 {
            (MODE as *mut u8).write(mode);
            return 0;
        }
        let clone: u32 = lf_checker_rt::callee_thiscall!(CLONE_INTO, u32, mgr);
        ((clone as *mut u8).wrapping_byte_offset(MODE as isize)).write(mode);
        clone
    }
});
