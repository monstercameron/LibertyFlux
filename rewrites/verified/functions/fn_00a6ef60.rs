// original: 0x00a6ef60 CEventAcquaintancePedDislike::vf21

/// Clone hook of the acquaintance-dislike event: forwards this event's
/// subject word to the event manager's clone routine and stamps the result.
///
/// Fetches the current event manager through the anchor at
/// `EVENT_MANAGER_ANCHOR` (callee 1); a null manager yields null. Otherwise
/// reads the subject word at `this + SUBJECT`, passes it with the manager
/// in ecx to the clone routine (callee 2), writes the vtable `VTABLE` at
/// the clone's head, and returns the clone.
///
/// Original: thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00a6ef60(this: u32) -> u32 {
    unsafe {
        const EVENT_MANAGER_ANCHOR: u32 = 0x0166_69c8;
        const SUBJECT: u32 = 0x18;
        const VTABLE: u32 = 0x00e9_ed04;
        const GET_MANAGER: u32 = 1;
        const CLONE_INTO: u32 = 2;

        let anchor = (lf_checker_rt::relocated(EVENT_MANAGER_ANCHOR) as *const u32)
            .read_unaligned();
        let mgr: u32 = lf_checker_rt::callee_thiscall!(GET_MANAGER, u32, anchor);
        if mgr == 0 {
            return 0;
        }
        let subject =
            ((this as *const u32).wrapping_byte_offset(SUBJECT as isize)).read_unaligned();
        lf_checker_rt::callee_thiscall!(CLONE_INTO, u32, mgr, subject);
        // The stamp is a file VA the loader relocates.
        (mgr as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        mgr
    }
});
