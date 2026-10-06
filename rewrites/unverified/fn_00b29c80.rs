// original: 0x00b29c80 rows_notify_tail_reset

/// Notifies the owner about every loaded row, then tail-calls the reset.
///
/// Walks the 2072 sixteen-byte rows in order; for each row whose data word
/// is nonzero calls the notify callee with (row index, owner handle). Then
/// tail-calls the slots-and-rows reset and returns its answer. Cdecl, no
/// stack words.
lf_checker_rt::export!(cdecl, rw_00b29c80() -> u32 {
    unsafe {
        const ROW_TABLE: u32 = 0x01657A10;
        const ROW_STRIDE: u32 = 16;
        const ROW_DATA: u32 = 4;
        const ROW_COUNT: u32 = 2072;
        const HANDLE: u32 = 0x0165FB98;
        const NOTIFY: u32 = 0;
        const RESET: u32 = 1;
        let table = lf_checker_rt::relocated(ROW_TABLE);
        let handle = (lf_checker_rt::relocated(HANDLE) as *const u32).read_unaligned();
        let mut i = 0u32;
        while i < ROW_COUNT {
            let row = table + i.wrapping_mul(ROW_STRIDE);
            let data = ((row + ROW_DATA) as *const u32).read_unaligned();
            if data != 0 {
                lf_checker_rt::callee_cdecl!(NOTIFY, u32, i, handle);
            }
            i += 1;
        }
        lf_checker_rt::callee_cdecl!(RESET, u32,)
    }
});
