// original: 0x00cb7820 shelter_subtask_select
/// Select a shelter subtask through a chain of type checks (4 calls).
///
/// Runs slot `0xC` of the table of the child at `[this + 8]` (thiscall,
/// one stack argument) and requires answer `0x11D`; requires the
/// grandchild at child `+0x14` to be non-null; runs its slot `0xC` and
/// requires `0x3B5`; runs the direct callee with (child, `a0`) and
/// requires non-null; runs the answer's slot `0xC` and requires `0x3B5`.
/// Returns the direct callee's answer when every check passes, else null.
/// All four callees are intercepted (three through planted tables) and
/// answered by the checker.
lf_checker_rt::export!(thiscall, rw_00cb7820(this: u32, a0: u32) -> u32 {
    unsafe {
        /// Child and grandchild offsets.
        const CHILD_OFF: u32 = 8;
        const GRAND_OFF: u32 = 0x14;
        /// Table slot of the three indirect checks.
        const SLOT: u32 = 0xC;
        /// Expected parent and child type ids.
        const PARENT_TYPE: u32 = 0x11D;
        const CHILD_TYPE: u32 = 0x3B5;
        /// Direct callee id.
        const SELECT: u32 = 3;
        type Slot0 = extern "thiscall" fn(u32) -> u32;
        let t1 = ((this + CHILD_OFF) as *const u32).read_unaligned();
        let va = (t1 as *const u32).read_unaligned();
        let sa = ((va + SLOT) as *const u32).read_unaligned();
        let fa: Slot0 = core::mem::transmute(sa as usize);
        if fa(t1) != PARENT_TYPE {
            return 0;
        }
        let t2 = ((t1 + GRAND_OFF) as *const u32).read_unaligned();
        if t2 == 0 {
            return 0;
        }
        let vb = (t2 as *const u32).read_unaligned();
        let sb = ((vb + SLOT) as *const u32).read_unaligned();
        let fb: Slot0 = core::mem::transmute(sb as usize);
        if fb(t2) != CHILD_TYPE {
            return 0;
        }
        let r: u32 = lf_checker_rt::callee_thiscall!(SELECT, u32, t1, a0);
        if r == 0 {
            return 0;
        }
        let vd = (r as *const u32).read_unaligned();
        let sd = ((vd + SLOT) as *const u32).read_unaligned();
        let fd: Slot0 = core::mem::transmute(sd as usize);
        if fd(r) != CHILD_TYPE {
            return 0;
        }
        r
    }
});
