// original: 0x00a6ef10 CTaskSimplePauseSystemTimer::vf1

/// Clone hook of the pause-system-timer task: builds a fresh task object and
/// carries over this task's timer word.
///
/// Fetches the current manager through the anchor at `MANAGER_ANCHOR`
/// (callee 1); a null manager yields null. Otherwise runs the base
/// initialiser (callee 2) with the fresh object in ecx, stamps the vtable
/// `VTABLE` at its head, zeroes the words at `+0x14` and `+0x18` and the
/// half-word at `+0x1c`, copies the timer word from `this + TIMER` to
/// `clone + TIMER`, and returns the clone.
///
/// Original: thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00a6ef10(this: u32) -> u32 {
    unsafe {
        const MANAGER_ANCHOR: u32 = 0x0167_e2a0;
        const VTABLE: u32 = 0x00e9_edbc;
        const TIMER: u32 = 0x20;
        const GET_MANAGER: u32 = 1;
        const BASE_INIT: u32 = 2;

        let anchor = (lf_checker_rt::relocated(MANAGER_ANCHOR) as *const u32).read_unaligned();
        let mgr: u32 = lf_checker_rt::callee_thiscall!(GET_MANAGER, u32, anchor);
        if mgr == 0 {
            return 0;
        }
        let timer = ((this as *const u32).wrapping_byte_offset(TIMER as isize)).read_unaligned();
        lf_checker_rt::callee_thiscall!(BASE_INIT, u32, mgr);
        // The stamp is a file VA the loader relocates.
        (mgr as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        ((mgr as *mut u32).wrapping_byte_offset(0x14)).write_unaligned(0);
        ((mgr as *mut u32).wrapping_byte_offset(0x18)).write_unaligned(0);
        ((mgr as *mut u16).wrapping_byte_offset(0x1c)).write_unaligned(0);
        ((mgr as *mut u32).wrapping_byte_offset(TIMER as isize)).write_unaligned(timer);
        mgr
    }
});
