// original: 0x00BE7CA0 task_activate_gated (proposed)
/// Set the active flag from a request byte, refreshing state on demand.
///
/// `obj` points to the task object, `request` selects the mode and
/// `param` is passed to the refresh callee. Only the low byte of
/// `request` is read: any value but 1 clears the flag byte at `+0x85`.
/// When it is 1 and the state word at `+0x38` is already set (not -1),
/// the flag is set without calling. Otherwise the refresh callee runs
/// (thiscall, one stack word) and the flag is set only if the state word
/// is no longer -1 afterwards; when the refresh leaves it at -1 the flag
/// keeps whatever it held. No result.
///
/// Original: 0x00BE7CA0 (thiscall, two stack words). The callee answers
/// and its state-word write are scripted by the checker.
lf_checker_rt::export!(thiscall, rw_00BE7CA0(obj: u32, request: u32, param: u32) -> u32 {
    unsafe {
        const STATE: u32 = 0x38;
        const ACTIVE: u32 = 0x85;
        const UNSET: u32 = 0xffff_ffff;
        const REFRESH: u32 = 1;
        if (request as u8) != 1 {
            ((obj + ACTIVE) as *mut u8).write(0);
            return 0;
        }
        if ((obj + STATE) as *const u32).read_unaligned() != UNSET {
            ((obj + ACTIVE) as *mut u8).write(1);
            return 0;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(REFRESH, u32, obj, param);
        if ((obj + STATE) as *const u32).read_unaligned() != UNSET {
            ((obj + ACTIVE) as *mut u8).write(1);
        }
        0
    }
});

