// original: 0x00b55be0 find_node_by_field44
/// Find the first node in the +0x1a28 list whose +0x44 field equals the key.
///
/// Nodes are linked through +0x8c and a node counts as typed when its u16 at
/// +0x48 equals 1; untyped nodes compare as value 0, so searching for key 0
/// also matches them. Returns the node base (node address + 4) or null.
export!(thiscall, rw_00b55be0(this: u32, key: u32) -> u32 {
    unsafe {
        const HEAD: u32 = 0x1a28;
        let mut node = ((this + HEAD) as *const u32).read();
        if node == 0 {
            return 0;
        }
        loop {
            let typed = ((node + 0x48) as *const u16).read() == 1;
            let base = node.wrapping_add(4);
            node = ((node + 0x8c) as *const u32).read();
            let value = if typed {
                ((base + 0x40) as *const u32).read()
            } else {
                0
            };
            if value == key {
                return base;
            }
            if node == 0 {
                return 0;
            }
        }
    }
});
