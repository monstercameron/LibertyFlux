// original: 0x00872CA0 node_resolve_unlink_release
/// Resolve a node through virtual slot `0x30`, then unlink and release it.
///
/// Forwards the argument to slot `0x30` of `this`'s table. A null answer
/// returns 0; an answer whose `+0x0C` word is not `this` returns the
/// answer itself. Otherwise the node is guarded (16-bit count at `+4` raised),
/// unlinked through the direct helper, handed off through its own slot 4,
/// and the guard dropped again: a surviving guard returns `0xFFFF`,
/// while a guard reaching zero deletes through the owner hook at `+0x18`
/// (direct helper, stdcall) or virtual slot 0 with argument 1, returning
/// the deleter's answer. All comparisons are exact equality or null
/// checks, with no signedness.
///
/// Original: 0x00872CA0 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00872CA0(this: u32, arg: u32) -> u32 {
    unsafe {
        const RESOLVE_SLOT: u32 = 0x30;
        const PARENT_OFF: u32 = 0x0C;
        const REF_OFF: u32 = 4;
        const HOOK_OFF: u32 = 0x18;
        const HANDOFF_SLOT: u32 = 4;
        const UNLINK_CALLEE: u32 = 2;
        const HOOK_CALLEE: u32 = 4;
        let vt = (this as *const u32).read_unaligned();
        let target = ((vt + RESOLVE_SLOT) as *const u32).read_unaligned();
        let resolve: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        let node = resolve(this, arg);
        if node == 0 {
            return 0;
        }
        let back = ((node + PARENT_OFF) as *const u32).read_unaligned();
        if back != this {
            return node;
        }
        let guard = (node + REF_OFF) as *mut u16;
        guard.write_unaligned(guard.read_unaligned().wrapping_add(1));
        lf_checker_rt::callee_thiscall!(UNLINK_CALLEE, u32, node);
        let vt2 = (node as *const u32).read_unaligned();
        let target2 = ((vt2 + HANDOFF_SLOT) as *const u32).read_unaligned();
        let handoff: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target2 as usize);
        handoff(node);
        let left = guard.read_unaligned().wrapping_add(0xFFFF);
        guard.write_unaligned(left);
        if left != 0 {
            return 0xFFFF;
        }
        let hook = ((node + HOOK_OFF) as *const u32).read_unaligned();
        if hook != 0 {
            lf_checker_rt::callee_stdcall!(HOOK_CALLEE, u32, node)
        } else {
            let vt3 = (node as *const u32).read_unaligned();
            let target3 = (vt3 as *const u32).read_unaligned();
            let delete: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target3 as usize);
            delete(node, 1)
        }
    }
});
