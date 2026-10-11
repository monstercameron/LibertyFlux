// original: 0x00B55F80 crmtManagerPriority_select_two_by_weight

/// Scans the linked records at `this + 0x1A28`, retaining the two highest
/// positive scores from active kind-1 records whose signed value at record
/// offset `0x08` lies in the inclusive `[min_value, max_value]` range. The
/// score is the `f32` at record offset `0x34`; strict comparisons keep the
/// earlier record on ties, and NaNs are not greater. Writes the two record
/// pointers and their scores to the four output pointers, in best then second
/// order, and returns `second_score_out`. Node links are at `+0x8C`, kind at
/// `+0x48`, and the record begins at `+4`. This is a thiscall method with six
/// stack arguments.
lf_checker_rt::export!(thiscall, rw_00b55f80(this: u32, min_value: i32, max_value: i32, best_out: u32, best_score_out: u32, second_out: u32, second_score_out: u32) -> u32 {
    unsafe {
        const MANAGER_HEAD: u32 = 0x1a28;
        const NODE_NEXT: u32 = 0x8c;
        const NODE_KIND: u32 = 0x48;
        const RECORD_FROM_NODE: u32 = 4;
        const RECORD_ACTIVE: u32 = 0x40;
        const RECORD_VALUE: u32 = 8;
        const RECORD_SCORE: u32 = 0x34;
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u16 { unsafe { (address as *const u16).read_unaligned() } }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }
        let mut node = read_u32(this.wrapping_add(MANAGER_HEAD));
        let mut best_record = 0;
        let mut second_record = 0;
        let mut best_score = 0.0f32;
        let mut second_score = 0.0f32;
        while node != 0 {
            let record = node.wrapping_add(RECORD_FROM_NODE);
            let next = read_u32(node.wrapping_add(NODE_NEXT));
            let value = read_u32(record.wrapping_add(RECORD_VALUE)) as i32;
            if read_u16(node.wrapping_add(NODE_KIND)) == 1
                && read_u32(record.wrapping_add(RECORD_ACTIVE)) != 0
                && value >= min_value
                && value <= max_value
            {
                let score = f32::from_bits(read_u32(record.wrapping_add(RECORD_SCORE)));
                if score > best_score {
                    second_record = best_record;
                    second_score = best_score;
                    best_record = record;
                    best_score = score;
                } else if score > second_score {
                    second_record = record;
                    second_score = score;
                }
            }
            node = next;
        }
        write_u32(best_out, best_record);
        write_u32(best_score_out, best_score.to_bits());
        write_u32(second_out, second_record);
        write_u32(second_score_out, second_score.to_bits());
        second_score_out
    }
});
