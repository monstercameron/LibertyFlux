// original: 0x00638200 reindex_child_rule_indices

/// Write each child rule's position into its word at offset `0x1C`, then
/// clear the parent's active-child count at offset `0x114`. The parent stores
/// a child-pointer array at `0x130` and its 16-bit length at `0x134`. A zero
/// length does not read the array pointer. The parent arrives in ECX; there
/// are no stack arguments or outgoing calls.
lf_checker_rt::export!(thiscall, rw_00638200(this: u32) -> u32 {
    unsafe {
        const CHILD_ARRAY: u32 = 0x130;
        const CHILD_COUNT: u32 = 0x134;
        const ACTIVE_COUNT: u32 = 0x114;
        const CHILD_INDEX: u32 = 0x1C;

        let count = (this.wrapping_add(CHILD_COUNT) as *const u16).read_unaligned() as u32;
        let mut index = 0u32;
        if index < count {
            let children = (this.wrapping_add(CHILD_ARRAY) as *const u32).read_unaligned();
            while index < count {
                let child = (children.wrapping_add(index.wrapping_mul(4)) as *const u32)
                    .read_unaligned();
                (child.wrapping_add(CHILD_INDEX) as *mut u32).write_unaligned(index);
                index = index.wrapping_add(1);
            }
        }
        (this.wrapping_add(ACTIVE_COUNT) as *mut u32).write_unaligned(0);
        index
    }
});
