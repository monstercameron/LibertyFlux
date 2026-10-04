// original: 0x00b55e40 find_first_live_node
/// Find the first live node in the +0x1a28 list and remember its successor.
///
/// Clears the +0x1a30 slot, then walks the list (link at +0x8c) for the first
/// node whose u16 at +0x48 is 1 with a non-zero +0x44 field. On a hit the
/// successor link is stored to +0x1a30 and the node address + 4 is returned;
/// otherwise null is returned with +0x1a30 left cleared.
export!(thiscall, rw_00b55e40(this: u32) -> u32 {
    unsafe {
        const HEAD: u32 = 0x1a28;
        const CURSOR: u32 = 0x1a30;
        let mut node = ((this + HEAD) as *const u32).read();
        ((this + CURSOR) as *mut u32).write(0);
        if node == 0 {
            return 0;
        }
        loop {
            let typed = ((node + 0x48) as *const u16).read() == 1;
            let base = node.wrapping_add(4);
            node = ((node + 0x8c) as *const u32).read();
            if typed && ((base + 0x40) as *const u32).read() != 0 {
                ((this + CURSOR) as *mut u32).write(node);
                return base;
            }
            if node == 0 {
                return 0;
            }
        }
    }
});
