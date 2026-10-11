// original: 0x00a23900 task_group_merge_flag_words

/// For every enabled task record, OR the two words immediately before its
/// destination pair into the pair. The four groups use signed counts, byte
/// enable flags and distinct record strides.
lf_checker_rt::export!(thiscall, rw_00a23900(this: u32) -> u32 {
    const GROUPS: [(u32, u32, u32, u32); 4] = [
        (0x400, 0x410, 0xd8, 0xb0),
        (0x404, 0x411, 0x188, 0xc0),
        (0x408, 0x412, 0x248, 0xe0),
        (0x40c, 0x413, 0x328, 0x120),
    ];
    unsafe {
        for (count_offset, flags_offset, destination_offset, stride) in GROUPS {
            let count = (this.wrapping_add(count_offset) as *const u32).read_unaligned() as i32;
            let mut index = 0i32;
            while index < count {
                if (this.wrapping_add(flags_offset).wrapping_add(index as u32) as *const u8).read() != 0 {
                    let destination = this.wrapping_add(destination_offset).wrapping_add((index as u32).wrapping_mul(stride));
                    let low = (destination.wrapping_sub(8) as *const u32).read_unaligned()
                        | (destination as *const u32).read_unaligned();
                    let high = (destination.wrapping_sub(4) as *const u32).read_unaligned()
                        | (destination.wrapping_add(4) as *const u32).read_unaligned();
                    (destination as *mut u32).write_unaligned(low);
                    (destination.wrapping_add(4) as *mut u32).write_unaligned(high);
                }
                index += 1;
            }
        }
    }
    0
});
