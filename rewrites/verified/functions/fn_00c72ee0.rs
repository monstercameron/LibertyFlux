// original: 0x00c72ee0 initialize_animation_lookup_tables

/// Reset the four animation-table counts, then initialize three pointer fields
/// in each of 160 consecutive 0x110-byte records. The pointer values refer to
/// three relocated arrays; the final EAX value is the first address at or
/// beyond the initialized record range.
lf_checker_rt::export!(cdecl, rw_00c72ee0() -> u32 {
    const COUNTS: [u32; 4] = [0x016d_d690, 0x016d_d694, 0x016d_d698, 0x016d_d69c];
    const TABLE_START: u32 = 0x016d_d79c;
    const TABLE_END: u32 = 0x016e_819c;
    const RECORD_STRIDE: u32 = 0x110;

    unsafe {
        for count_address in COUNTS {
            (lf_checker_rt::relocated(count_address) as *mut u32).write_unaligned(0);
        }

        let mut field = lf_checker_rt::relocated(TABLE_START);
        let end = lf_checker_rt::relocated(TABLE_END);
        while field < end {
            ((field.wrapping_sub(4)) as *mut u32)
                .write_unaligned(lf_checker_rt::relocated(0x016e_80a0));
            (field as *mut u32).write_unaligned(lf_checker_rt::relocated(0x016e_8aa0));
            ((field.wrapping_add(4)) as *mut u32)
                .write_unaligned(lf_checker_rt::relocated(0x016e_99a0));
            field = field.wrapping_add(RECORD_STRIDE);
        }
        field
    }
});
