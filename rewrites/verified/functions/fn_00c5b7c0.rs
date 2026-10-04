// original: 0x00c5b7c0 task_maybe_notify_kind2 (proposed)

/// Notify the task system about an object unless it is filtered out.
///
/// The object is skipped when the word at `+0xb30` is nonzero while bit 2 of
/// the byte at `+0x26c` is set, when the flag byte at `+0x210` is set with
/// the mode word at `+0xa74` equal to 1 or 2, or when the link at `+0x224` is
/// null. Otherwise callee 1 is invoked with (`this`, 2, `obj`) and its answer
/// is returned; on the skip paths the stale `obj` pointer is returned as the
/// original leaves it in `eax`.
///
/// Original: 0x00c5b7c0 (thiscall: `this` in ecx, one stack word).
lf_checker_rt::export!(thiscall, rw_00c5b7c0(this: u32, obj: u32) -> u32 {
    unsafe {
        const NOTIFY: u32 = 1;
        const KIND: u32 = 2;
        if ((obj + 0xb30) as *const u32).read_unaligned() != 0
            && ((obj + 0x26c) as *const u8).read() & 4 != 0
        {
            return obj;
        }
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

