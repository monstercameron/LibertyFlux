// original: 0x00b55b70 find_node_by_field18
/// Find the first live node in the +0x1a28 list whose +0x18 field equals the key.
///
/// A node is live when its u16 at +0x48 equals 1 and its +0x44 field is
/// non-zero. Key 0xFFFFFFFF matches nothing. Returns node address + 4 or null.
export!(thiscall, rw_00b55b70(this: u32, key: u32) -> u32 {
    unsafe {
        const HEAD: u32 = 0x1a28;
        if key == 0xffff_ffff {
            return 0;
        }
        let mut node = ((this + HEAD) as *const u32).read();
        if node == 0 {
            return 0;
        }
        loop {
            let typed = ((node + 0x48) as *const u16).read() == 1;
            let base = node.wrapping_add(4);
            node = ((node + 0x8c) as *const u32).read();
            if typed
                && ((base + 0x40) as *const u32).read() != 0
                && ((base + 0x14) as *const u32).read() == key
            {
                return base;
            }
            if node == 0 {
                return 0;
            }
        }
    }
});
