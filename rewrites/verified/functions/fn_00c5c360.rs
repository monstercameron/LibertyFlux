// original: 0x00c5c360 task_swap_tracked_ref (proposed)

/// Swap the tracked reference held by a task owner, unless filtered out.
///
/// The object is skipped when the flag byte at `+0x210` is set with mode 1
/// or 2 at `+0xa74`, or when its link at `+0x224` is null. Otherwise, when
/// the word at `+0xb30` is nonzero and bit 2 of the byte at `+0x26c` is set,
/// callee 1 is notified with kind 0 if the byte at `+0xa60` is 2, the old
/// reference at `this+0x58` is released (callee 2) when non-null, the slot
/// takes the word at `+0xb30`, and a reference is added (callee 3). On the
/// remaining path callee 1 is notified with kind 0 directly. Returns nothing.
///
/// Original: 0x00c5c360 (thiscall: `this` in ecx, one stack word).
lf_checker_rt::export!(thiscall, rw_00c5c360(this: u32, obj: u32) -> u32 {
    unsafe {
        const SLOT: u32 = 0x58;
        const NOTIFY: u32 = 1;
        const RELEASE: u32 = 2;
        const ADDREF: u32 = 3;
        if ((obj + 0x210) as *const u8).read() != 0 {
            let mode = ((obj + 0xa74) as *const u32).read_unaligned();
            if mode == 1 || mode == 2 {
                return 0;
            }
        }
        if ((obj + 0x224) as *const u32).read_unaligned() == 0 {
            return 0;
        }
        if ((obj + 0xb30) as *const u32).read_unaligned() != 0
            && ((obj + 0x26c) as *const u8).read() & 4 != 0
        {
            if ((obj + 0xa60) as *const u8).read() == 2 {
                lf_checker_rt::callee_thiscall!(NOTIFY, u32, this, 0, obj);
            }
            let held = ((this + SLOT) as *const u32).read_unaligned();
            if held != 0 {
                lf_checker_rt::callee_thiscall!(RELEASE, u32, held, this + SLOT);
            }
            let track = ((obj + 0xb30) as *const u32).read_unaligned();
            ((this + SLOT) as *mut u32).write_unaligned(track);
            lf_checker_rt::callee_thiscall!(ADDREF, u32, track, this + SLOT);
            return 0;
        }
        lf_checker_rt::callee_thiscall!(NOTIFY, u32, this, 0, obj);
        0
    }
});

