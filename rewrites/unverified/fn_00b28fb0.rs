// original: 0x00b28fb0 rows_notify_match

/// Notifies the owner about every ready row with a given id.
///
/// Walks the 2072 sixteen-byte rows in order; for each row whose head word
/// equals `id`, whose data word is nonzero and whose flag byte is clear,
/// calls the notify callee with (row index, owner handle). Returns the last
/// callee answer. Cdecl, one stack word.
lf_checker_rt::export!(cdecl, rw_00b28fb0(id: u32) -> u32 {
    unsafe {
        const ROW_TABLE: u32 = 0x01657A10;
        const ROW_STRIDE: u32 = 16;
        const ROW_HEAD: u32 = 0;
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
            let head = ((row + ROW_HEAD) as *const u32).read_unaligned();
            let data = ((row + ROW_DATA) as *const u32).read_unaligned();
            let flag = ((row + ROW_FLAG) as *const u8).read();
            if head == id && data != 0 && flag == 0 {
                r = lf_checker_rt::callee_cdecl!(NOTIFY, u32, i, handle);
            }
            i += 1;
        }
        r
    }
});
