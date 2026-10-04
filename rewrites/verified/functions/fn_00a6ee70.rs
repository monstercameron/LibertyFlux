// original: 0x00a6ee70 CTaskComplexPlayerPlaceCarBomb::vf1

/// Clone hook of the place-car-bomb player task: forwards this task's stored
/// parameter to the task manager's clone routine.
///
/// Fetches the current manager through the anchor at `MANAGER_ANCHOR`
/// (callee 1); a null manager yields null. Otherwise reads the parameter
/// word at `this + PARAM`, passes it with the manager in ecx to the clone
/// routine (callee 2), and returns its result.
///
/// Original: thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00a6ee70(this: u32) -> u32 {
    unsafe {
        const MANAGER_ANCHOR: u32 = 0x0167_e2a0;
        const PARAM: u32 = 0x14;
        const GET_MANAGER: u32 = 1;
        const CLONE_INTO: u32 = 2;

        let anchor = (lf_checker_rt::relocated(MANAGER_ANCHOR) as *const u32).read_unaligned();
        let mgr: u32 = lf_checker_rt::callee_thiscall!(GET_MANAGER, u32, anchor);
        if mgr == 0 {
            return 0;
        }
        let param = ((this as *const u32).wrapping_byte_offset(PARAM as isize)).read_unaligned();
        lf_checker_rt::callee_thiscall!(CLONE_INTO, u32, mgr, param)
    }
});
