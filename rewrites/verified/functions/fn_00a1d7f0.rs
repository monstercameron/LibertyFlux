// original: 0x00a1d7f0 task_group_release_enabled_records

/// Visit the four variable-length task groups and call the release helper for
/// each record whose enable byte is nonzero. Counts are signed, so zero and
/// negative counts skip a group. Each helper call receives its record in ECX.
lf_checker_rt::export!(thiscall, rw_00a1d7f0(this: u32) -> u32 {
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
                    let _ = lf_checker_rt::callee_thiscall!(1, u32, task);
                }
                index += 1;
            }
        }
    }
    0
});
