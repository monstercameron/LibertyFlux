// original: 0x00cf43c0 climb_param_pair_lookup (proposed)

/// Looks up a pair of float parameters for a climb task id and stores them
/// through two out-pointers. Both slots are zeroed first; ids `0x69..=0x74`
/// then select one of four constant pairs through a jump table, while any
/// other id (or a table entry that points at the default arm) leaves the
/// zeros in place.
///
/// Returns the table's case index for ids inside the range, and the
/// id-minus-base value itself for ids outside it.
///
/// Original: 0x00cf43c0 (stdcall, three stack words: id, out_first, out_second).
lf_checker_rt::export!(stdcall, rw_00cf43c0(task_id: u32, out_first: u32, out_second: u32) -> u32 {
    const RANGE_BASE: u32 = 0x69;
    const RANGE_LAST_OFFSET: u32 = 0xb;
    // (first, second) constant pairs, as raw bits.
    const PAIR_A: (u32, u32) = (0x3ecc_cccd, 0xbfaf_5c29);
    const PAIR_B: (u32, u32) = (0x3f40_0000, 0xbf66_6666);
    const PAIR_C: (u32, u32) = (0x3f0c_cccd, 0x0000_0000);
    const PAIR_D: (u32, u32) = (0x3f20_0000, 0xbf5c_28f6);
    // Jump-table case index per id offset 0..=11; 4 is the "keep zeros" arm.
    const CASES: [u32; 12] = [0, 4, 4, 1, 2, 4, 4, 4, 4, 4, 3, 3];

    unsafe {
        let shifted = task_id.wrapping_sub(RANGE_BASE);
        (out_first as *mut u32).write_unaligned(0);
        (out_second as *mut u32).write_unaligned(0);
        if shifted > RANGE_LAST_OFFSET {
            return shifted;
        }
        let case = CASES[shifted as usize];
        let pair = match case {
            0 => PAIR_A,
            1 => PAIR_D,
            2 => PAIR_B,
            3 => PAIR_C,
            _ => return case,
        };
        (out_first as *mut u32).write_unaligned(pair.0);
        (out_second as *mut u32).write_unaligned(pair.1);
        case
    }
});
