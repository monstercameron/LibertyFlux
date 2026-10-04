// original: 0x00a71040 CTaskComplexPlayerOnFoot::vf19

/// Dispatch hook of the on-foot player task: either clones through the
/// manager or advances the task's pending event, depending on the mode byte.
///
/// When the mode byte at `this + MODE` is non-zero, fetches the manager
/// (callee 1) and, unless it is null, runs the shared clone routine
/// (callee 2, reached by a tail jump in the original that first overwrites
/// the incoming argument slot with the mode byte) with the manager in ecx
/// and the mode byte as its word argument, returning its result. A null
/// manager yields null.
///
/// When the mode byte is zero, reads the event flags at `arg + EVENT_FLAGS`.
/// If flag bit 0 is set, clears it in memory and runs the event advance
/// routine (callee 3, `this` in ecx, four words: `arg, 0, 0, 1`); a non-zero
/// result is returned at once. Otherwise, and when the bit was clear, runs the
/// default advance routine (callee 4) with `this` in ecx and the words
/// `arg, 8`, returning its result.
///
/// Original: thiscall, one stack word (`arg`), callee pops 4.
lf_checker_rt::export!(thiscall, rw_00a71040(this: u32, arg: u32) -> u32 {
    unsafe {
        const MANAGER_ANCHOR: u32 = 0x0167_e2a0;
        const MODE: u32 = 0x50;
        const EVENT_FLAGS: u32 = 0x270;
        const ADVANCE_KIND: u32 = 8;
        const GET_MANAGER: u32 = 1;
        const SHARED_CLONE: u32 = 2;
        const ADVANCE_EVENT: u32 = 3;
        const ADVANCE_DEFAULT: u32 = 4;

        let mode = ((this as *const u8).wrapping_byte_offset(MODE as isize)).read();
        if mode != 0 {
            let anchor =
                (lf_checker_rt::relocated(MANAGER_ANCHOR) as *const u32).read_unaligned();
            let mgr: u32 = lf_checker_rt::callee_thiscall!(GET_MANAGER, u32, anchor);
            if mgr == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(SHARED_CLONE, u32, mgr, mode as u32);
        }
        let flags_at = (arg as *mut u32).wrapping_byte_offset(EVENT_FLAGS as isize);
        let flags = flags_at.read_unaligned();
        if flags & 1 != 0 {
            flags_at.write_unaligned(flags & 0xffff_fffe);
            let advanced: u32 =
                lf_checker_rt::callee_thiscall!(ADVANCE_EVENT, u32, this, arg, 0, 0, 1);
            if advanced != 0 {
                return advanced;
            }
        }
        lf_checker_rt::callee_thiscall!(ADVANCE_DEFAULT, u32, this, arg, ADVANCE_KIND)
    }
});
