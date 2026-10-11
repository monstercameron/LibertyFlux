// original: 0x00a1dbd0 task_group_sum_enabled_record_values

/// Visit enabled records in four signed-count task groups and add each
/// helper's unsigned result to a wrapping 32-bit total. The group order,
/// record addresses and call order follow the object layout.
lf_checker_rt::export!(thiscall, rw_00a1dbd0(this: u32) -> u32 {
    const GROUPS: [(u32, u32, u32, u32); 4] = [
        (0x400, 0x410, 0x90, 0xb0),
        (0x404, 0x411, 0x140, 0xc0),
        (0x408, 0x412, 0x200, 0xe0),
        (0x40c, 0x413, 0x2e0, 0x120),
    ];
    let mut total = 0u32;
    unsafe {
        for (count_offset, flags_offset, first_offset, stride) in GROUPS {
            let count = (this.wrapping_add(count_offset) as *const u32).read_unaligned() as i32;
            let mut index = 0i32;
            while index < count {
                if (this.wrapping_add(flags_offset).wrapping_add(index as u32) as *const u8).read() != 0 {
                    let task = this.wrapping_add(first_offset).wrapping_add((index as u32).wrapping_mul(stride));
                    total = total.wrapping_add(lf_checker_rt::callee_thiscall!(1, u32, task));
                }
                index += 1;
            }
        }
    }
    total
});
