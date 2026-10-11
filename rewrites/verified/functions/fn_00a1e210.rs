// original: 0x00a1e210 task_group_find_first_active_result

/// Visit the four task groups in their stored order and ask the activity
/// helper about each enabled task. Counts are signed; stop and return one at
/// the first nonzero low-byte result, otherwise return zero after all groups.
lf_checker_rt::export!(thiscall, rw_00a1e210(this: u32) -> u32 {
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
                let enabled = (this.wrapping_add(flags_offset).wrapping_add(index as u32) as *const u8).read();
                if enabled != 0 {
                    let task = this.wrapping_add(first_offset).wrapping_add((index as u32).wrapping_mul(stride));
                    if lf_checker_rt::callee_thiscall!(1, u32, task) as u8 != 0 {
                        return 1;
                    }
                }
                index += 1;
            }
        }
    }
    0
});
