// original: 0x00be7b20 task_apply_float_chain (proposed)

/// Push a float value through a chained list of applier objects.
///
/// `this` points to the task, `ped` to the ped, `bits` is the float bits.
/// When the gate poll (callee 1, thiscall with `this`, one word: the ped)
/// answers non-zero, resolves the list head through callee 2 (thiscall on
/// the word at `ped+0x224` plus 0x44, one word holding 1): fetches each
/// object's peer through virtual slot 0x30 (callee 3, thiscall, no stack
/// words) and applies the float through the peer's virtual slot 4 (callee 4,
/// thiscall, two words: the float bits and 1), first for the head, then for
/// each link at `+0x08` (skipping the apply when the peer is null), and
/// finally for the object at `this+0x14`. No meaningful return value
/// (`ret: none`). Twin of the sibling at 0xBE7BC0 (integer variant).
///
/// Original: thiscall, two stack words, the callee pops 8 bytes.
lf_checker_rt::export!(thiscall, rw_00be7b20(this: u32, ped: u32, bits: u32) -> u32 {
    unsafe {
        const PED_OWNER: u32 = 0x224;
        const OWNER_INNER: u32 = 0x44;
        const OFF_FINAL: u32 = 0x14;
        const OFF_NEXT: u32 = 0x08;
        const SLOT_PEER: u32 = 0x30;
        const SLOT_APPLY: u32 = 0x04;
        const GATE: u32 = 1;
        const RESOLVE: u32 = 2;

        let peer_of = |obj: u32| -> u32 {
            unsafe {
                let vtable = (obj as *const u32).read_unaligned();
                let fetch: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                    (((vtable + SLOT_PEER)) as *const u32).read_unaligned() as usize,
                );
                fetch(obj)
            }
        };
        let apply_to = |peer: u32| {
            unsafe {
                let vtable = (peer as *const u32).read_unaligned();
                let apply: extern "thiscall" fn(u32, u32, u32) -> u32 = core::mem::transmute(
                    (((vtable + SLOT_APPLY)) as *const u32).read_unaligned() as usize,
                );
                apply(peer, bits, 1);
            }
        };

        let go: u32 = lf_checker_rt::callee_thiscall!(GATE, u32, this, ped);
        if go as u8 != 0 {
            let owner = ((ped + PED_OWNER) as *const u32).read_unaligned();
            let head: u32 =
                lf_checker_rt::callee_thiscall!(RESOLVE, u32, owner.wrapping_add(OWNER_INNER), 1);
            apply_to(peer_of(head));
            let mut node = ((head + OFF_NEXT) as *const u32).read_unaligned();
            while node != 0 {
                let peer = peer_of(node);
                if peer != 0 {
                    apply_to(peer_of(node));
                }
                node = ((node + OFF_NEXT) as *const u32).read_unaligned();
            }
        }
        let fin = ((this + OFF_FINAL) as *const u32).read_unaligned();
        apply_to(peer_of(fin));
        0
    }
});
