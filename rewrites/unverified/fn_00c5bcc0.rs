// original: 0x00c5bcc0 task_maybe_notify_kind1 (proposed)

/// Notify the task system about an object unless it is filtered out.
///
/// The object is skipped when the flag byte at `+0x210` is set and the mode
/// word at `+0xa74` is 1 or 2, or when its link at `+0x224` is null.
/// Otherwise callee 1 is invoked with the entry `ecx` passed through and
/// arguments (1, `obj`), and its answer is returned; on the skip paths the
/// stale `obj` pointer is returned as the original leaves it in `eax`.
///
/// Original: 0x00c5bcc0 (thiscall: entry ecx passed through, one stack word).
lf_checker_rt::export!(thiscall, rw_00c5bcc0(this: u32, obj: u32) -> u32 {
    unsafe {
        const NOTIFY: u32 = 1;
        const KIND: u32 = 1;
        if ((obj + 0x210) as *const u8).read() != 0 {
            let mode = ((obj + 0xa74) as *const u32).read_unaligned();
            if mode == 1 || mode == 2 {
                return obj;
            }
        }
        if ((obj + 0x224) as *const u32).read_unaligned() == 0 {
            return obj;
        }
        lf_checker_rt::callee_thiscall!(NOTIFY, u32, this, KIND, obj)
    }
});

