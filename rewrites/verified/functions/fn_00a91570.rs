// original: 0x00a91570 stream_close_four_children (proposed)

/// Close up to four child objects, then tail into the parent's own close.
///
/// Each nonzero child pointer in the four slots at `this+0x14` is closed by
/// recursion (callee 1, thiscall/0 on the child) and then detached (callee 2,
/// thiscall/1 with the child on the object from the set's global). Finally
/// the object at `this+0x10` is closed by tail call (callee 3, thiscall/0),
/// whose answer is the return.
///
/// Returns the tail call's answer. Thiscall: object in ecx, no stack words;
/// the original ends in a jump, the rewrite in a forwarding call.
lf_checker_rt::export!(thiscall, rw_00a91570(this: u32) -> u32 {
    unsafe {
        const CHILDREN: u32 = 0x14;
        const NCHILD: u32 = 4;
        const PARENT: u32 = 0x10;
        const SET_GLOBAL: u32 = 0x012fb254;
        const RECURSE_CALLEE: u32 = 1;
        const DETACH_CALLEE: u32 = 2;
        const TAIL_CALLEE: u32 = 3;
        for k in 0..NCHILD {
            let child =
                ((this + CHILDREN + k * 4) as *const u32).read_unaligned();
            if child != 0 {
                lf_checker_rt::callee_thiscall!(RECURSE_CALLEE, u32, child);
                let set = lf_checker_rt::global::<u32>(SET_GLOBAL).read_unaligned();
                lf_checker_rt::callee_thiscall!(DETACH_CALLEE, u32, set, child);
            }
        }
        lf_checker_rt::callee_thiscall!(TAIL_CALLEE, u32, this + PARENT)
    }
});
