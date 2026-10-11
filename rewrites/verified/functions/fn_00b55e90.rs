// original: 0x00B55E90 crmtManagerPriority_find_by_compare

/// Searches the priority manager's linked records for the first active item
/// of kind 1 whose signed value at record offset `0x08` matches `threshold`.
/// `comparison` is an unsigned selector: 0 means `<`, 1 means `<=`, 2 means
/// `==`, 3 means `>=`, and 4 means `>`; values above 4 never match. The head
/// pointer is at manager offset `0x1A28`; each node stores its next pointer at
/// `0x8C`, its kind at `0x48`, and its record begins at `+4`. A match advances
/// the manager cursor at `0x1A30` to the next node and returns the record
/// pointer. A miss returns zero and leaves the cursor unchanged.
lf_checker_rt::export!(thiscall, rw_00b55e90(this: u32, threshold: i32, comparison: u32) -> u32 {
    unsafe {
        const MANAGER_HEAD: u32 = 0x1a28;
        const MANAGER_CURSOR: u32 = 0x1a30;
        const NODE_NEXT: u32 = 0x8c;
        const NODE_KIND: u32 = 0x48;
        const RECORD_FROM_NODE: u32 = 4;
        const RECORD_ACTIVE: u32 = 0x40;
        const RECORD_VALUE: u32 = 8;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u16 {
            unsafe { (address as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let mut node = read_u32(this.wrapping_add(MANAGER_HEAD));
        while node != 0 {
            let record = node.wrapping_add(RECORD_FROM_NODE);
            let next = read_u32(node.wrapping_add(NODE_NEXT));
            if read_u16(node.wrapping_add(NODE_KIND)) == 1
                && read_u32(record.wrapping_add(RECORD_ACTIVE)) != 0
                && comparison <= 4
            {
                let value = read_u32(record.wrapping_add(RECORD_VALUE)) as i32;
                let matches = match comparison {
                    0 => value < threshold,
                    1 => value <= threshold,
                    2 => value == threshold,
                    3 => value >= threshold,
                    4 => value > threshold,
                    _ => false,
                };
                if matches {
                    write_u32(this.wrapping_add(MANAGER_CURSOR), next);
                    return record;
                }
            }
            node = next;
        }
        0
    }
});
