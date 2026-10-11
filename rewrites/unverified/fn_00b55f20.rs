// original: 0x00B55F20 crmtManagerPriority_find_highest_for_key

/// Returns the active kind-1 record with the greatest positive score at
/// record offset `0x58` among nodes whose key at `0x08` equals `key`. The
/// manager head is at `this + 0x1A28`; each node links through `+0x8C`, stores
/// its kind at `+0x48`, and has its record at `+4`. `exclude_record` is
/// skipped by pointer equality. Scores are compared as `f32`; zero, negative
/// values, and unordered NaNs do not replace the initial zero score, and a
/// tie keeps the first record. Returns zero if none qualifies. This is a
/// thiscall method with two stack arguments.
lf_checker_rt::export!(thiscall, rw_00b55f20(this: u32, key: u32, exclude_record: u32) -> u32 {
    unsafe {
        const MANAGER_HEAD: u32 = 0x1a28;
        const NODE_NEXT: u32 = 0x8c;
        const NODE_KIND: u32 = 0x48;
        const RECORD_FROM_NODE: u32 = 4;
        const RECORD_ACTIVE: u32 = 0x40;
        const RECORD_KEY: u32 = 8;
        const RECORD_SCORE: u32 = 0x58;
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u16 { unsafe { (address as *const u16).read_unaligned() } }
        let mut node = read_u32(this.wrapping_add(MANAGER_HEAD));
        let mut best_record = 0;
        let mut best_score = 0.0f32;
        while node != 0 {
            let record = node.wrapping_add(RECORD_FROM_NODE);
            let next = read_u32(node.wrapping_add(NODE_NEXT));
            if record != exclude_record
                && read_u16(node.wrapping_add(NODE_KIND)) == 1
                && read_u32(record.wrapping_add(RECORD_ACTIVE)) != 0
                && read_u32(record.wrapping_add(RECORD_KEY)) == key
            {
                let score = f32::from_bits(read_u32(record.wrapping_add(RECORD_SCORE)));
                if score > best_score {
                    best_record = record;
                    best_score = score;
                }
            }
            node = next;
        }
        best_record
    }
});
