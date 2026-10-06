// original: 0x009543D0 table_byte_clear (proposed)

/// Clear the tag byte of every armed record in a fixed table.
///
/// Walks `COUNT` (0x400) records of `STRIDE` (0x14) bytes starting at
/// `TABLE`. For each record it reads a flag dword 0x10 bytes below the
/// record start, the byte at record+1 and the tag byte at record+0; when
/// all three are nonzero it writes 0 to the tag byte. Always runs all
/// 1024 iterations. No return value (cdecl/0, original leaves a loop
/// leftover in EAX, compared as `none`).
lf_checker_rt::export!(cdecl, rw_009543D0() -> u32 {
    const TABLE: u32 = 0x0120F2C8;
    const COUNT: u32 = 0x400;
    const STRIDE: u32 = 0x14;
    const FLAG_BACK: u32 = 0x10;
    let mut p = lf_checker_rt::relocated(TABLE);
    let mut n = COUNT;
    while n != 0 {
        let flag = unsafe { (p.wrapping_sub(FLAG_BACK) as *const u32).read_unaligned() };
        let b1 = unsafe { (p.wrapping_add(1) as *const u8).read() };
        let b0 = unsafe { (p as *const u8).read() };
        if flag != 0 && b1 != 0 && b0 != 0 {
            unsafe { (p as *mut u8).write(0) };
        }
        p = p.wrapping_add(STRIDE);
        n -= 1;
    }
    0
});
