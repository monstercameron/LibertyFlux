// original: 0x00B56010 crmtManagerPriority_select_two_by_flags_and_weight

/// Selects from the linked records at `this + 0x1A28` when the node kind is
/// 1, the record is active, and its signed value at record offset `0x08` is
/// inside the inclusive bounds. The record's flags at `+0x04` qualify when
/// `((flags & flag_mask) != 0)` equals whether the low byte of
/// `require_flag` is nonzero. A strictly greater primary `f32` at `+0x34`
/// becomes best and shifts the previous best to second. Otherwise, the
/// secondary `f32` at `+0x58` is compared with the current second score; if
/// greater, that record becomes second and its primary score is stored there.
/// Writes best/second pointers and scores, then returns `second_score_out`.
/// Nodes link at `+0x8C`, store kind at `+0x48`, and contain the record at
/// `+4`. This is a thiscall method with eight stack arguments.
lf_checker_rt::export!(thiscall, rw_00b56010(this: u32, min_value: i32, max_value: i32, flag_mask: u32, require_flag: u32, best_out: u32, best_score_out: u32, second_out: u32, second_score_out: u32) -> u32 {
    unsafe {
        const MANAGER_HEAD: u32 = 0x1a28;
        const NODE_NEXT: u32 = 0x8c;
        const NODE_KIND: u32 = 0x48;
        const RECORD_FROM_NODE: u32 = 4;
        const RECORD_FLAGS: u32 = 4;
        const RECORD_ACTIVE: u32 = 0x40;
        const RECORD_VALUE: u32 = 8;
        const RECORD_PRIMARY: u32 = 0x34;
        const RECORD_SECONDARY: u32 = 0x58;
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u16 { unsafe { (address as *const u16).read_unaligned() } }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }
        let require_flag = (require_flag as u8) != 0;
        let mut node = read_u32(this.wrapping_add(MANAGER_HEAD));
        let mut best_record = 0;
        let mut second_record = 0;
        let mut best_primary = 0.0f32;
        let mut second_score = 0.0f32;
        while node != 0 {
            let record = node.wrapping_add(RECORD_FROM_NODE);
            let next = read_u32(node.wrapping_add(NODE_NEXT));
            let value = read_u32(record.wrapping_add(RECORD_VALUE)) as i32;
            let flags_match = read_u32(record.wrapping_add(RECORD_FLAGS)) & flag_mask != 0;
            if read_u16(node.wrapping_add(NODE_KIND)) == 1
                && read_u32(record.wrapping_add(RECORD_ACTIVE)) != 0
                && value >= min_value
                && value <= max_value
                && flags_match == require_flag
            {
                let primary = f32::from_bits(read_u32(record.wrapping_add(RECORD_PRIMARY)));
                if primary > best_primary {
                    second_record = best_record;
                    second_score = best_primary;
                    best_record = record;
                    best_primary = primary;
                } else {
                    let secondary = f32::from_bits(read_u32(record.wrapping_add(RECORD_SECONDARY)));
                    if secondary > second_score {
                        second_record = record;
                        second_score = primary;
                    }
                }
            }
            node = next;
        }
        write_u32(best_out, best_record);
        write_u32(best_score_out, best_primary.to_bits());
        write_u32(second_out, second_record);
        write_u32(second_score_out, second_score.to_bits());
        second_score_out
    }
});
