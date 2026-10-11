// original: 0x00a239f0 task_group_visit_enabled_records_with_context

/// Visit enabled records across four task groups in stored order. Counts are
/// signed and the helper receives each record in ECX plus the unchanged
/// context word on the stack.
lf_checker_rt::export!(thiscall, rw_00a239f0(this: u32, context: u32) -> u32 {
    const GROUPS: [(u32, u32, u32, u32); 4] = [
        (0x400, 0x410, 0x90, 0xb0),
        (0x404, 0x411, 0x140, 0xc0),
        (0x408, 0x412, 0x200, 0xe0),
        (0x40c, 0x413, 0x2e0, 0x120),
    ];
    unsafe {
        for (count_offset, flags_offset, first_offset, stride) in GROUPS {
            let count = (this.wrapping_add(count_offset) as *const u32).read_unaligned() as i32;
            let mut index = 0i32;
            while index < count {
                if (this.wrapping_add(flags_offset).wrapping_add(index as u32) as *const u8).read() != 0 {
                    let record = this.wrapping_add(first_offset).wrapping_add((index as u32).wrapping_mul(stride));
                    let _ = lf_checker_rt::callee_thiscall!(1, u32, record, context);
                }
                index += 1;
            }
        }
    }
    0
});
