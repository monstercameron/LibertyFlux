// original: 0x00872770 node_reparent
/// Reparent a node: unlink it, then attach or delete both sides.
///
/// Guards the incoming node (16-bit count at `+4` raised) and unlinks it
/// through the direct helper. When `this + 0x0C` is set, the triple
/// (`this`, node, flag) goes to the direct link helper and the node guard
/// is dropped. Otherwise, a set low flag byte runs the handoff through
/// virtual slot 4 of `this`'s table, then one reference of `this` is
/// released (deleting through the owner hook or slot 0 when the count
/// reaches zero); the node is guarded once more and stored into the
/// owner's `+8` slot. The node guard is finally dropped on both paths,
/// deleting the node the same way when it reaches zero. No return value:
/// `eax` on exit always holds a callee's answer, never function output.
/// Guard arithmetic is 16-bit wrapping; the flag gate reads the low byte
/// only.
///
/// Original: 0x00872770 (thiscall, two stack arguments).
lf_checker_rt::export!(thiscall, rw_00872770(this: u32, node: u32, flag: u32) -> u32 {
    unsafe {
        const REF_OFF: u32 = 4;
        const LINK_OFF: u32 = 0x0C;
        const OWNER_OFF: u32 = 8;
        const HOOK_OFF: u32 = 0x18;
        const HANDOFF_SLOT: u32 = 4;
        const UNLINK_CALLEE: u32 = 1;
        const ATTACH_CALLEE: u32 = 2;
        const HOOK_CALLEE: u32 = 4;
        let guard = (node + REF_OFF) as *mut u16;
        guard.write_unaligned(guard.read_unaligned().wrapping_add(1));
        lf_checker_rt::callee_thiscall!(UNLINK_CALLEE, u32, node);
        let link = ((this + LINK_OFF) as *const u32).read_unaligned();
        if link != 0 {
            lf_checker_rt::callee_stdcall!(ATTACH_CALLEE, u32, this, node, flag);
        } else {
            let owner = ((this + OWNER_OFF) as *const u32).read_unaligned();
            if (flag & 0xFF) != 0 {
                let vt = (this as *const u32).read_unaligned();
                let target = ((vt + HANDOFF_SLOT) as *const u32).read_unaligned();
                let handoff: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(target as usize);
                handoff(this);
            }
            let slot = (this + REF_OFF) as *mut u16;
            let left = slot.read_unaligned().wrapping_add(0xFFFF);
            slot.write_unaligned(left);
            if left == 0 {
                let hook = ((this + HOOK_OFF) as *const u32).read_unaligned();
                if hook != 0 {
                    lf_checker_rt::callee_stdcall!(HOOK_CALLEE, u32, this);
                } else {
                    let vt = (this as *const u32).read_unaligned();
                    let target = (vt as *const u32).read_unaligned();
                    let delete: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(target as usize);
                    delete(this, 1);
                }
            }
            guard.write_unaligned(guard.read_unaligned().wrapping_add(1));
            ((owner + OWNER_OFF) as *mut u32).write_unaligned(node);
        }
        let left = guard.read_unaligned().wrapping_add(0xFFFF);
        guard.write_unaligned(left);
        if left == 0 {
            let hook = ((node + HOOK_OFF) as *const u32).read_unaligned();
            if hook != 0 {
                lf_checker_rt::callee_stdcall!(HOOK_CALLEE, u32, node);
            } else {
                let vt = (node as *const u32).read_unaligned();
                let target = (vt as *const u32).read_unaligned();
                let delete: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(target as usize);
                delete(node, 1);
            }
        }
    }
    0
});
