// original: 0x00B30380 recording_category

/// Convert a recording identifier into its internal category. Zero and
/// signed IDs above 47 return three without a call. Other values, including
/// negative IDs, reach the helper. Read its record's tag at +4; tags 1 through 8
/// map to 4, 6, 7, 8, 10, 9, 11 and 5 respectively, and all other tags return
/// three. The internal dispatch is a local jump table; the only outgoing call
/// is a caller-cleaned cdecl helper with one word argument.
lf_checker_rt::export!(cdecl, rw_b30380(record_id: u32) -> u32 {
    unsafe {
        if record_id == 0 || (record_id as i32) > 0x2f {
            return 3;
        }
        let record = lf_checker_rt::callee_cdecl!(1, u32, record_id);
        let tag = (record.wrapping_add(4) as *const u32).read_unaligned();
        match tag {
            1 => 4,
            2 => 6,
            3 => 7,
            4 => 8,
            5 => 10,
            6 => 9,
            7 => 11,
            8 => 5,
            _ => 3,
        }
    }
});
