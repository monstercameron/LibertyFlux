// original: 0x00a21ba0 task_groups_call_initialization_helper

/// Call the helper once for the root record, then for every record in four
/// counted groups. The groups are visited in the order 0x404, 0x400, 0x408,
/// 0x40c; each count is signed and each helper receives its record in ECX.
lf_checker_rt::export!(thiscall, rw_00a21ba0(this: u32) -> u32 {
    const GROUPS: [(u32, u32, u32); 4] = [
        (0x404, 0x140, 0xc0),
        (0x400, 0x90, 0xb0),
        (0x408, 0x200, 0xe0),
        (0x40c, 0x2e0, 0x120),
    ];
    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
        for (count_offset, first_offset, stride) in GROUPS {
            let count = (this.wrapping_add(count_offset) as *const u32).read_unaligned() as i32;
            let mut index = 0i32;
            while index < count {
                let record = this.wrapping_add(first_offset).wrapping_add((index as u32).wrapping_mul(stride));
                let _ = lf_checker_rt::callee_thiscall!(1, u32, record);
                index += 1;
            }
        }
    }
    0
});
