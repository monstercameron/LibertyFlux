// original: 0x00872D00 node_queue_drain
/// Drain the `+0x1C` node queue, deleting each unreferenced node.
///
/// Walks the queue headed at `this + 0x1C` (linked through `+0x10`) and
/// runs virtual slot 4 on every node, then drops one 16-bit reference at
/// `+4`: survivors continue down the saved link. A node whose count
/// reaches zero is deleted: without an owner hook at `+0x18` through its
/// own slot 0 with argument 1; with one, the hook's critical section at
/// `+8` is entered when set, the direct transfer helper runs on the hook
/// with the word at `+6`, a non-null transfer result re-registers the
/// node through slot 0 with argument 0 (stamping the returned table word
/// into the node, raising the 16-bit count at `+6` of the registration
/// behind it, and linking the node into its `+0x0C` slot), and the
/// section is left when still set. The head is nulled at the end.
/// Returns `this`.
///
/// Original: 0x00872D00 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00872D00(this: u32) -> u32 {
    unsafe {
        const HEAD_OFF: u32 = 0x1C;
        const NEXT_OFF: u32 = 0x10;
        const REF_OFF: u32 = 4;
        const HOOK_OFF: u32 = 0x18;
        const SECTION_OFF: u32 = 8;
        const STEP_SLOT: u32 = 4;
        const ENTER_CALLEE: u32 = 2;
        const TRANSFER_CALLEE: u32 = 3;
        const LEAVE_CALLEE: u32 = 5;
        let mut node = ((this + HEAD_OFF) as *const u32).read_unaligned();
        while node != 0 {
            let vt = (node as *const u32).read_unaligned();
            let next = ((node + NEXT_OFF) as *const u32).read_unaligned();
            let target = ((vt + STEP_SLOT) as *const u32).read_unaligned();
            let step: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            step(node);
            let slot = (node + REF_OFF) as *mut u16;
            let left = slot.read_unaligned().wrapping_add(0xFFFF);
            slot.write_unaligned(left);
            if left == 0 {
                let hook = ((node + HOOK_OFF) as *const u32).read_unaligned();
                if hook != 0 {
                    let section = hook + SECTION_OFF;
                    if ((section) as *const u32).read_unaligned() != 0 {
                        lf_checker_rt::callee_stdcall!(ENTER_CALLEE, u32, section);
                    }
                    let kind =
                        ((node + 6) as *const u16).read_unaligned() as u32;
                    let moved: u32 =
                        lf_checker_rt::callee_thiscall!(TRANSFER_CALLEE, u32, hook, kind);
                    if moved != 0 {
                        let vt2 = (node as *const u32).read_unaligned();
                        let target2 = (vt2 as *const u32).read_unaligned();
                        let release: extern "thiscall" fn(u32, u32) -> u32 =
                            core::mem::transmute(target2 as usize);
                        release(node, 0);
                        let reg = ((moved + 4) as *const u32).read_unaligned();
                        let word =
                            ((reg + 0x0C) as *const u32).read_unaligned();
                        (node as *mut u32).write_unaligned(word);
                        let cnt = (reg + 6) as *mut u16;
                        cnt.write_unaligned(cnt.read_unaligned().wrapping_add(1));
                        ((reg + 0x0C) as *mut u32).write_unaligned(node);
                    }
                    if ((section) as *const u32).read_unaligned() != 0 {
                        lf_checker_rt::callee_stdcall!(LEAVE_CALLEE, u32, section);
                    }
                } else {
                    let vt3 = (node as *const u32).read_unaligned();
                    let target3 = (vt3 as *const u32).read_unaligned();
                    let delete: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(target3 as usize);
                    delete(node, 1);
                }
            }
            node = next;
        }
        ((this + HEAD_OFF) as *mut u32).write_unaligned(0);
    }
    this
});
