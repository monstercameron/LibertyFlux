// original: 0x006F7540 mainloop_timer_secondary_tree_insert

/// Insert the supplied node into the secondary tagged tree by its 16-bit
/// key. The tree uses child and parent links at offsets 0x4C, 0x50 and 0x54,
/// with the key at 0x58. The low 16 bits of `new_key - node_key` select the
/// right link when their sign bit is set and the left link otherwise. After
/// insertion the node is marked, the balancing helper is called, the key is
/// copied into the node, the tree count is incremented, and the result is
/// written to the first stack argument.
lf_checker_rt::export!(thiscall, rw_006f7540(tree: u32, result: u32, key_ptr: u32, node: u32) -> u32 {
    unsafe {
        const ROOT: u32 = 0x00;
        const COUNT: u32 = 0x04;
        const LEFT: u32 = 0x4c;
        const RIGHT: u32 = 0x50;
        const PARENT: u32 = 0x54;
        const KEY: u32 = 0x58;
        const BALANCE_CALLEE: u32 = 1;
        unsafe fn read32(address: u32) -> u32 { (address as *const u32).read_unaligned() }
        unsafe fn write32(address: u32, value: u32) { (address as *mut u32).write_unaligned(value); }
        unsafe fn read16(address: u32) -> u16 { (address as *const u16).read_unaligned() }
        unsafe fn write16(address: u32, value: u16) { (address as *mut u16).write_unaligned(value); }

        let new_key = read16(key_ptr);
        let mut current = read32(tree + ROOT);
        let mut inserted = 0u32;
        if current == 0 {
            write32(tree + ROOT, node);
            inserted = node;
        } else {
            while current != 0 {
                let node_key = read16(current + KEY);
                let difference = new_key.wrapping_sub(node_key);
                if difference & 0x8000 != 0 {
                    let right = read32(current + RIGHT);
                    if right != 0 {
                        current = right;
                    } else {
                        write32(current + RIGHT, node);
                        write32(node + PARENT, current);
                        inserted = node;
                        break;
                    }
                } else {
                    let tagged_left = read32(current + LEFT);
                    let left = tagged_left & !1;
                    if left != 0 {
                        current = left;
                    } else {
                        write32(current + LEFT, left | (tagged_left & 1) | node);
                        write32(node + PARENT, current);
                        inserted = node;
                        break;
                    }
                }
            }
        }

        if inserted != 0 {
            let flags = read32(node + LEFT);
            write32(node + LEFT, flags | 1);
            let _helper_result: u32 = lf_checker_rt::callee_thiscall!(BALANCE_CALLEE, u32, tree, node);
            write16(node + KEY, read16(key_ptr));
            write32(tree + COUNT, read32(tree + COUNT).wrapping_add(1));
        }
        write16(result, if inserted == 0 { 0 } else { read16(inserted + KEY) });
        write32(result + 4, inserted);
        write32(result + 8, tree);
        ((result + 12) as *mut u8).write(u8::from(inserted != 0));
    }
    0
});
