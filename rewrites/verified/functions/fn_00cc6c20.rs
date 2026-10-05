// original: 0x00CC6C20 euphoria_kind_table (proposed)

/// Look up the static id table and entry count for a feedback category.
///
/// `kind` selects one of three table/count pairs: 2 gives the 12-entry
/// table, 3 the 10-entry table, 4 the 3-entry table. Any other value stores a
/// null pointer and returns zero. The chosen (possibly null) table address is
/// always stored through `out`, and the entry count is returned. The
/// subtraction chain wraps, so values below 2 fall through to the default.
///
/// Original: 0x00CC6C20 (stdcall, two stack words).
lf_checker_rt::export!(stdcall, rw_00cc6c20(kind: u32, out: u32) -> u32 {
    unsafe {
        const TABLE_12: u32 = 0x0105152C;
        const COUNT_12: u32 = 12;
        const TABLE_10: u32 = 0x0105155C;
        const COUNT_10: u32 = 10;
        const TABLE_3: u32 = 0x01051584;
        const COUNT_3: u32 = 3;
        let mut probe = kind.wrapping_sub(2);
        if probe == 0 {
            (out as *mut u32).write_unaligned(lf_checker_rt::relocated(TABLE_12));
            return COUNT_12;
        }
        probe = probe.wrapping_sub(1);
        if probe == 0 {
            (out as *mut u32).write_unaligned(lf_checker_rt::relocated(TABLE_10));
            return COUNT_10;
        }
        probe = probe.wrapping_sub(1);
        if probe == 0 {
            (out as *mut u32).write_unaligned(lf_checker_rt::relocated(TABLE_3));
            return COUNT_3;
        }
        (out as *mut u32).write_unaligned(0);
        0
    }
});
