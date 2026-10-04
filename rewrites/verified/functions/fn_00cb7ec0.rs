// original: 0x00cb7ec0 shelter_task_query
/// Query a shelter task through a chain of type checks (3 calls).
///
/// Runs slot `0xC` of the table of the child at `[this + 8]` (thiscall,
/// two stack arguments) and requires answer `0x11D`; a null child returns
/// the pinned incoming eax cleared (see contract). Runs the direct callee
/// with (child, `a0`) and requires non-null; runs the answer's slot `0xC`
/// and requires `0x3B8`. With a zero low byte in `a1` returns bit 0 of
/// `[this + 0x74]`; otherwise returns the last answer with its low byte
/// forced to 1. Any failed check returns the checked value cleared, or
/// null. All three callees are intercepted (two through planted tables).
lf_checker_rt::export!(thiscall, rw_00cb7ec0(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        /// Child offset and table slot of the two indirect checks.
        const CHILD_OFF: u32 = 8;
        const SLOT: u32 = 0xC;
        /// Status word holding the reported bit.
        const STATUS_OFF: u32 = 0x74;
        /// Expected parent and child type ids.
        const PARENT_TYPE: u32 = 0x11D;
        const CHILD_TYPE: u32 = 0x3B8;
        /// Pinned incoming eax (see contract).
        const IN_EAX: u32 = 0xA5A5A5A5;
        /// Direct callee id.
        const QUERY: u32 = 2;
        type Slot0 = extern "thiscall" fn(u32) -> u32;
        let t1 = ((this + CHILD_OFF) as *const u32).read_unaligned();
        if t1 == 0 {
            return IN_EAX & 0xFFFFFF00;
        }
        let va = (t1 as *const u32).read_unaligned();
        let sa = ((va + SLOT) as *const u32).read_unaligned();
        let fa: Slot0 = core::mem::transmute(sa as usize);
        let r1 = fa(t1);
        if r1 != PARENT_TYPE {
            return r1 & 0xFFFFFF00;
        }
        let r: u32 = lf_checker_rt::callee_thiscall!(QUERY, u32, t1, a0);
        if r == 0 {
            return 0;
        }
        let vc = (r as *const u32).read_unaligned();
        let sc = ((vc + SLOT) as *const u32).read_unaligned();
        let fc: Slot0 = core::mem::transmute(sc as usize);
        let r3 = fc(r);
        if r3 != CHILD_TYPE {
            return r3 & 0xFFFFFF00;
        }
        if (a1 & 0xFF) == 0 {
            ((this + STATUS_OFF) as *const u32).read_unaligned() & 1
        } else {
            (r3 & 0xFFFFFF00) | 1
        }
    }
});
