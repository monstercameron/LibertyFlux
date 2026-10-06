// original: 0x00b28c60 slot_find_write_rowval

/// Finds a file slot by id and copies its row's head word to the caller.
///
/// Writes 0xFFFFFFFF to `*out` first, then scans the 24 slots in order for
/// the first slot whose flag byte is nonzero and whose id word equals `id`.
/// On a hit reads that slot's row index, loads the head word of the
/// 16-byte row (rows start at ROW_TABLE, stride 16), writes it to `*out` and
/// returns it with its low byte set to 1. On a miss returns 0. Cdecl, two
/// stack words.
lf_checker_rt::export!(cdecl, rw_00b28c60(id: u32, out: u32) -> u32 {
    unsafe {
        const SLOT_FLAGS: u32 = 0x01657890;
        const SLOT_IDS: u32 = 0x01657650;
        const SLOT_ROWS: u32 = 0x016579B0;
        const ROW_TABLE: u32 = 0x01657A10;
        const ROW_STRIDE: u32 = 16;
        const SLOT_COUNT: u32 = 24;
        const MISS_VAL: u32 = 0xFFFF_FFFF;
        (out as *mut u32).write_unaligned(MISS_VAL);
        let flags = lf_checker_rt::relocated(SLOT_FLAGS);
        let ids = lf_checker_rt::relocated(SLOT_IDS);
        let rows = lf_checker_rt::relocated(SLOT_ROWS);
        let table = lf_checker_rt::relocated(ROW_TABLE);
        let mut i = 0u32;
        while i < SLOT_COUNT {
            let flag = ((flags + i) as *const u8).read();
            let cur = ((ids + i * 4) as *const u32).read_unaligned();
            if flag != 0 && cur == id {
                let r = ((rows + i * 4) as *const u32).read_unaligned();
                let v = ((table + r.wrapping_mul(ROW_STRIDE)) as *const u32).read_unaligned();
                (out as *mut u32).write_unaligned(v);
                return (v & 0xFFFF_FF00) | 1;
            }
            i += 1;
        }
        0
    }
});
