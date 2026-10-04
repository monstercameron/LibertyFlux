// original: 0x00874820 locked_node_lookup_and_unlink
/// Looks up a child node by id while holding the node's lock, unlinks the
/// child's current sibling link, releases the link's old target and records
/// the node as the sibling's new owner. Returns the unlinked sibling, or
/// null when the lookup fails.
export!(thiscall, rw_00874820(node: u32, sel: u32) -> u32 {
    unsafe {
        let cs = node + 8;
        if ((cs) as *const u32).read() != 0 {
            let slot = global::<u32>(0x00E731CC);
            let enter: extern "stdcall" fn(u32) -> u32 =
                core::mem::transmute(slot.read() as usize);
            enter(cs);
        }
        let mut out = 0u32;
        let found = callee_thiscall!(1, u32, node, sel);
        if found != 0 {
            let link = ((found + 4) as *const u32).read();
            if ((link + 0xC) as *const u32).read() != 0 {
                let sibling = ((link + 0xC) as *const u32).read();
                let next = (sibling as *const u32).read();
                ((link + 0xC) as *mut u32).write(next);
                let count = ((link + 6) as *mut u16).read().wrapping_sub(1);
                ((link + 6) as *mut u16).write(count);
                let vtable = (found as *const u32).read();
                let release: extern "cdecl" fn(u32) -> u32 = core::mem::transmute(
                    ((vtable + 0xC) as *const u32).read() as usize,
                );
                release(sibling);
                if sibling != 0 {
                    ((sibling + 0x18) as *mut u32).write(node);
                    out = sibling;
                }
            }
        }
        if ((cs) as *const u32).read() != 0 {
            let slot = global::<u32>(0x00E731C8);
            let leave: extern "stdcall" fn(u32) -> u32 =
                core::mem::transmute(slot.read() as usize);
            leave(cs);
        }
        out
    }
});
