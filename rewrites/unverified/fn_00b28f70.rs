// original: 0x00b28f70 rows_notify_ready

/// Notifies the owner about every ready row.
///
/// Walks the 2072 sixteen-byte rows in order; for each row whose data word
/// is nonzero and whose flag byte is clear, calls the notify callee with
/// (row index, owner handle). Returns the last callee answer. Cdecl, no
/// stack words.
lf_checker_rt::export!(cdecl, rw_00b28f70() -> u32 {
    unsafe {
        const ROW_TABLE: u32 = 0x01657A10;
        const ROW_STRIDE: u32 = 16;
        const ROW_DATA: u32 = 4;
        const ROW_FLAG: u32 = 12;
        const ROW_COUNT: u32 = 2072;
        const HANDLE: u32 = 0x0165FB98;
        const NOTIFY: u32 = 0;
        let table = lf_checker_rt::relocated(ROW_TABLE);
        let handle = (lf_checker_rt::relocated(HANDLE) as *const u32).read_unaligned();
        let mut r = 0u32;
        let mut i = 0u32;
        while i < ROW_COUNT {
            let row = table + i.wrapping_mul(ROW_STRIDE);
            let data = ((row + ROW_DATA) as *const u32).read_unaligned();
            let flag = ((row + ROW_FLAG) as *const u8).read();
            if data != 0 && flag == 0 {
                r = lf_checker_rt::callee_cdecl!(NOTIFY, u32, i, handle);
            }
            i += 1;
        }
        r
    }
});
