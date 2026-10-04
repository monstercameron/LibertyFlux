// original: 0x008749b0 find_slot_by_id
/// Looks up a slot in a motion-node table by id: ids below 0x80 index the
/// table directly, larger ids are found by scanning the overflow entries for
/// one whose doubly-indirected key matches. Returns the entry, or null when
/// nothing matches.
export!(thiscall, rw_008749b0(node: u32, sel: u32) -> u32 {
    unsafe {
        let table = (node as *const u32).read();
        if sel < 0x80 {
            ((table + sel * 4) as *const u32).read()
        } else {
            let count = ((node + 4) as *const u16).read() as u32;
            if count <= 0x17 {
                0
            } else {
                let mut i = 0x17u32;
                loop {
                    let entry = ((table + 0x5c + (i - 0x17) * 4) as *const u32).read();
                    let target = (entry as *const u32).read();
                    if ((target) as *const u32).read() == sel {
                        return ((table + i * 4) as *const u32).read();
                    }
                    i += 1;
                    if i >= count {
                        return 0;
                    }
                }
            }
        }
    }
});
