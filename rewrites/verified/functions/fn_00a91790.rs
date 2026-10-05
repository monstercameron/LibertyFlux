// original: 0x00a91790 stream_notify_value_deep (proposed)

/// Notify the set when it holds the value, then notify every child.
///
/// The `+0x10` list is searched for a node whose head equals the argument;
/// on a hit the set is notified (callee 1, thiscall/1 on `this+0x10` with
/// the value). Then each nonzero child slot past `+0x14` is notified the
/// same way by recursion (callee 2, thiscall/1 on the child with the value).
///
/// Returns the last callee answer, or 0 when nothing ran. Thiscall: object
/// in ecx, the value on the stack, callee pops 4.
lf_checker_rt::export!(thiscall, rw_00a91790(this: u32, val: u32) -> u32 {
    unsafe {
        const LIST_OFF: u32 = 0x10;
        const CHILDREN: u32 = 0x14;
        const NCHILD: u32 = 4;
        let mut ret = 0u32;
        let mut s = ((this + LIST_OFF) as *const u32).read_unaligned();
        while s != 0 {
            if ((s) as *const u32).read_unaligned() == val {
                ret = lf_checker_rt::callee_thiscall!(1, u32, this + LIST_OFF, val);
                break;
            }
            s = ((s + 4) as *const u32).read_unaligned();
        }
        for k in 0..NCHILD {
            let child =
                ((this + CHILDREN + k * 4) as *const u32).read_unaligned();
            if child != 0 {
                ret = lf_checker_rt::callee_thiscall!(2, u32, child, val);
            }
        }
        ret
    }
});
