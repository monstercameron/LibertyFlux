// original: 0x00b55dd0 find_node_by_bitmask
/// Find the first live node whose +0x0c flags overlap the mask, honouring flag.
///
/// Walks the +0x1a28 list (link at +0x8c) for a live node (u16 at +0x48 is 1,
/// +0x44 field non-zero) where "flags & mask != 0" equals the wanted flag
/// byte (1 = must overlap, 0 = must not). On a hit the successor link is
/// stored to +0x1a30 and the node address + 4 is returned, else null.
export!(thiscall, rw_00b55dd0(this: u32, mask: u32, flag: u32) -> u32 {
    unsafe {
        const HEAD: u32 = 0x1a28;
        const CURSOR: u32 = 0x1a30;
        let mut node = ((this + HEAD) as *const u32).read();
        ((this + CURSOR) as *mut u32).write(0);
        if node == 0 {
            return 0;
        }
        let want = (flag & 0xff) as u8;
        loop {
            let typed = ((node + 0x48) as *const u16).read() == 1;
            let base = node.wrapping_add(4);
            node = ((node + 0x8c) as *const u32).read();
            if typed && ((base + 0x40) as *const u32).read() != 0 {
                let hit = ((((base + 4) as *const u32).read() & mask) != 0) as u8;
                if hit == want {
                    ((this + CURSOR) as *mut u32).write(node);
                    return base;
                }
            }
            if node == 0 {
                return 0;
            }
        }
    }
});
