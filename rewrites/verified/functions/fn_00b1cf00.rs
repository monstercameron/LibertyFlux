// original: 0x00b1cf00 find_free_row (proposed)

/// Finds the first free row of the row table and records its index.
///
/// Thiscall with no stack arguments. Scans rows 0..94 (stride 0xE2 from
/// the row table): the first row whose kind word is below 0xBB, whose
/// state word is zero and whose state byte is 0 or 0x0E wins, and its
/// index is stored to the byte at +0x40 (pre-cleared, so a failed search
/// leaves 0). Returns the winning row's address, or the table end when
/// no row is free.
lf_checker_rt::export!(thiscall, rw_00b1cf00(this: u32) -> u32 {
    unsafe {
        const ROW_TABLE: u32 = 0x010401f2;
        const ROW_STRIDE: u32 = 0xe2;
        const ROW_COUNT: u32 = 94;
        const STATE_TABLE: u32 = 0x016334c8;
        const KIND_LIMIT: u16 = 0xbb;
        ((this + 0x40) as *mut u8).write(0);
        let mut i = 0u32;
        while i < ROW_COUNT {
            let row = ROW_TABLE.wrapping_add(i.wrapping_mul(ROW_STRIDE));
            let kind = (lf_checker_rt::relocated(row) as *const u16).read_unaligned();
            if kind < KIND_LIMIT {
                let state = STATE_TABLE.wrapping_add(i.wrapping_mul(4));
                let sword =
                    (lf_checker_rt::relocated(state + 2) as *const u16).read_unaligned();
                if sword == 0 {
                    let sbyte =
                        (lf_checker_rt::relocated(state) as *const u8).read();
                    if sbyte == 0 || sbyte == 0x0e {
                        ((this + 0x40) as *mut u8).write(i as u8);
                        return lf_checker_rt::relocated(row);
                    }
                }
            }
            i += 1;
        }
        lf_checker_rt::relocated(ROW_TABLE.wrapping_add(ROW_COUNT.wrapping_mul(ROW_STRIDE)))
    }
});
