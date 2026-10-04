// original: 0x00b55c20 find_node_by_field_pair
/// Find the first live node whose +0x10/+0x14 fields equal (key1, key0).
///
/// Same list walk as the other finders: nodes hang off +0x1a28 via +0x8c and
/// are live when the u16 at +0x48 is 1 with a non-zero +0x44 field. Note the
/// argument order: the second argument matches +0x10, the first matches +0x14.
/// Returns node address + 4 or null.
export!(thiscall, rw_00b55c20(this: u32, key0: u32, key1: u32) -> u32 {
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
            if typed
                && ((base + 0x40) as *const u32).read() != 0
                && ((base + 0x0c) as *const u32).read() == key1
                && ((base + 0x10) as *const u32).read() == key0
            {
                return base;
            }
            if node == 0 {
                return 0;
            }
        }
    }
});
