// original: 0x00a6ee00 CTaskComplexPlayerInCover::vf1

/// Clone hook of the in-cover player task: forwards this task's variant byte
/// to the task manager's clone routine.
///
/// Fetches the current manager through the anchor at `MANAGER_ANCHOR`
/// (callee 1, called with the anchor's value in ecx); a null manager means
/// there is nothing to clone into and the result is null. Otherwise reads
/// the variant byte at `this + VARIANT`, passes it (zero-extended) with the
/// manager in ecx to the clone routine (callee 2), and returns its result.
///
/// Original: thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00a6ee00(this: u32) -> u32 {
    unsafe {
        const MANAGER_ANCHOR: u32 = 0x0167_e2a0;
        const VARIANT: u32 = 0x18;
        const GET_MANAGER: u32 = 1;
        const CLONE_INTO: u32 = 2;

        let anchor = (lf_checker_rt::relocated(MANAGER_ANCHOR) as *const u32).read_unaligned();
        let mgr: u32 = lf_checker_rt::callee_thiscall!(GET_MANAGER, u32, anchor);
        if mgr == 0 {
            return 0;
        }
        let variant = ((this as *const u8).wrapping_byte_offset(VARIANT as isize)).read();
        lf_checker_rt::callee_thiscall!(CLONE_INTO, u32, mgr, variant as u32)
    }
});
