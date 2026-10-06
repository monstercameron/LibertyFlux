// original: 0x00872B20 node_index_lookup
/// Look up a motion-tree node by index.
///
/// The index arrives in `ecx` and is compared SIGNED throughout. Below 23
/// (0x17) it addresses the pointer table directly: the table base is read
/// from its global and the indexed slot returned unread. Otherwise the
/// table is scanned from slot 23 up to (excluding) the count held as a
/// 16-bit word in the second global; the first non-null slot whose pointed
/// node carries the index in its first word is returned. Anything else,
/// including a count at or below 23, yields null.
///
/// Original: 0x00872B20 (thiscall convention, index in `ecx`).
lf_checker_rt::export!(thiscall, rw_00872B20(index: u32) -> u32 {
    unsafe {
        const TABLE_PTR: u32 = 0x1B4AF2C;
        const COUNT_WORD: u32 = 0x1B4AF30;
        const DIRECT_MAX: i32 = 0x17;
        let idx = index as i32;
        let base = lf_checker_rt::global::<u32>(TABLE_PTR).read_unaligned();
        if idx < DIRECT_MAX {
            let slot = base.wrapping_add((idx as u32).wrapping_mul(4));
            return (slot as *const u32).read_unaligned();
        }
        let count = lf_checker_rt::global::<u16>(COUNT_WORD).read_unaligned() as i32;
        if count <= DIRECT_MAX {
            return 0;
        }
        let mut i = DIRECT_MAX;
        while i < count {
            let node = (base.wrapping_add((i as u32).wrapping_mul(4)) as *const u32)
                .read_unaligned();
            if node != 0 {
                let id = (node as *const u32).read_unaligned();
                if id == index {
                    return node;
                }
            }
            i += 1;
        }
        0
    }
});
