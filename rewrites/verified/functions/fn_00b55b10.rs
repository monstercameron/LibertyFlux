// original: 0x00b55b10 find_typed_descendant
/// Find the first descendant subtree whose root has type tag 4.
///
/// A null node yields null, and a node whose u16 tag at +0x06 is already 4
/// yields itself. Otherwise the node's function table is consulted: slot 8
/// must approve (non-zero low byte) and slot 9 gives the child count; each
/// child from slot 0x0c is then searched recursively in order, and the first
/// hit wins. Returns the matching node or null. The recursion is real on
/// both sides (the self-call is not intercepted); only the table slots are
/// stubbed, since their targets need live game state.
export!(cdecl, rw_00b55b10(node: u32) -> u32 {
    unsafe {
        if node == 0 {
            return 0;
        }
        if ((node + 6) as *const u16).read() == 4 {
            return node;
        }
        let vtable = (node as *const u32).read();
        let approve: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute((((vtable + 0x20) as *const u32).read()) as usize);
        if approve(node) & 0xff == 0 {
            return 0;
        }
        let count_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute((((vtable + 0x24) as *const u32).read()) as usize);
        let count = count_of(node) as i32;
        if count <= 0 {
            return 0;
        }
        let child_slot = ((vtable + 0x30) as *const u32).read();
        let mut i: i32 = 0;
        while i < count {
            let child_of: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(child_slot as usize);
            let hit = rw_00b55b10(child_of(node, i as u32));
            if hit != 0 {
                return hit;
            }
            i += 1;
        }
        0
    }
});
