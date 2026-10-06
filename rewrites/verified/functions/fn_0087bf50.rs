// original: 0x0087bf50 rage::crmtNodeProxy::vf1
/// Tear down a proxy node: notify the child, run the shared teardown.
///
/// When the child at `this+0x24` is non-null, notifies intercepted callee 1
/// (thiscall/1) with the address of the field at `this+0x1c`, then always
/// runs intercepted callee 2 (thiscall/0) over this, zeroes `+0x0c` and
/// `+0x10`, and zeroes `+0x08` when it is non-zero. Returns callee 2's
/// answer. All comparisons are null checks.
///
/// Original: thiscall/0, two direct calls, no floating point.
export!(thiscall, rw_0087bf50(this: u32) -> u32 {
    /// Child link, null-checked before the notify call.
    const CHILD_OFF: u32 = 0x24;
    /// Field whose address is passed to the notify call.
    const NOTIFY_OFF: u32 = 0x1C;
    unsafe {
        let child = ((this + CHILD_OFF) as *const u32).read_unaligned();
        if child != 0 {
            callee_thiscall!(1, u32, child, this.wrapping_add(NOTIFY_OFF));
        }
        let r = callee_thiscall!(2, u32, this);
        ((this + 0x0C) as *mut u32).write_unaligned(0);
        ((this + 0x10) as *mut u32).write_unaligned(0);
        if ((this + 0x08) as *const u32).read_unaligned() != 0 {
            ((this + 0x08) as *mut u32).write_unaligned(0);
        }
        r
    }
});
