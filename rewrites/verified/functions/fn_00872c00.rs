// original: 0x00872C00 rage::crmtNodeParent::vf1
/// Destructor body behind `crmtNodeParent::vf1`: one teardown pass.
///
/// Runs the direct child-list pass and the direct teardown helper on
/// `this`, then clears the link words at `+0x0C` and `+0x10` (plus `+8`
/// when non-null). No return value.
///
/// Original: 0x00872C00 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00872C00(this: u32) -> u32 {
    unsafe {
        const LIST_CALLEE: u32 = 1;
        const TEARDOWN_CALLEE: u32 = 2;
        lf_checker_rt::callee_thiscall!(LIST_CALLEE, u32, this);
        lf_checker_rt::callee_thiscall!(TEARDOWN_CALLEE, u32, this);
        let has_link = ((this + 8) as *const u32).read_unaligned() != 0;
        ((this + 0x0C) as *mut u32).write_unaligned(0);
        ((this + 0x10) as *mut u32).write_unaligned(0);
        if has_link {
            ((this + 8) as *mut u32).write_unaligned(0);
        }
    }
    0
});
