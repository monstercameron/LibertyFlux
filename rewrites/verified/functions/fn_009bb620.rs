// original: 0x009bb620 register_input_slot (proposed)

/// Register a value in the first free input slot of the 27-entry table.
///
/// Scans the table at 0x01291DE0 (27 entries of 0x30 bytes) for the first
/// entry whose flag byte (`+0x00`) is zero. When one is found at index `i`,
/// writes the 16-bit `code` at `+0x04`, sets the flag to 1, stores `weight`
/// at `+0x08`, copies four words from `vec` to `+0x10`..`+0x1c` and four
/// words from `mtx` to `+0x20`..`+0x2c`, and returns `i`. When every flag is
/// non-zero writes nothing and returns -1. All multi-byte moves are bitwise;
/// no floating-point arithmetic happens.
///
/// Edge cases: a full table returns -1 with no writes; only the flag bytes
/// steer the scan, so slot contents never matter for the search.
///
/// Original: cdecl, four stack words (`code`, `vec`, `mtx`, `weight`).
lf_checker_rt::export!(cdecl, rw_009bb620(code: u32, vec: u32, mtx: u32, weight: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x01291de0;
        const END: u32 = 0x012923e0;
        const STRIDE: u32 = 0x30;
        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let end = lf_checker_rt::relocated(END);
        let mut cur = lf_checker_rt::relocated(TABLE);
        let mut idx: u32 = 0;
        while cur < end {
            if (cur as *const u8).read() == 0 {
                ((cur + 0x04) as *mut u16).write_unaligned(code as u16);
                (cur as *mut u8).write(1);
                wr(cur + 0x08, weight);
                wr(cur + 0x10, rd(vec));
                wr(cur + 0x14, rd(vec + 0x04));
                wr(cur + 0x18, rd(vec + 0x08));
                wr(cur + 0x1c, rd(vec + 0x0c));
                wr(cur + 0x20, rd(mtx));
                wr(cur + 0x24, rd(mtx + 0x04));
                wr(cur + 0x28, rd(mtx + 0x08));
                wr(cur + 0x2c, rd(mtx + 0x0c));
                return idx;
            }
            idx += 1;
            cur += STRIDE;
        }
        0xFFFFFFFF
    }
});
