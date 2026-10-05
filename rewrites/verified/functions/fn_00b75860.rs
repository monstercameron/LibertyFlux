// original: 0x00b75860 pool_table_clear (proposed)

/// Clear the pool table header and one flag bit down the whole table.
///
/// Zeroes four header bytes at 0x167ca18..0x167ca1b and the words at
/// 0x167ca10 and 0x167ca14, clears bit 4 of one byte per 44-byte entry from
/// 0x1670d29 up to (not including) 0x167ca39, then zeroes the word at
/// 0x167ca1c. Returns the final cursor (the end address).
///
/// Original: 0x00b75860 (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_00b75860() -> u32 {
    unsafe {
        const HEAD_A_VA: u32 = 0x167ca10;
        const HEAD_B_VA: u32 = 0x167ca14;
        const HEAD_C_VA: u32 = 0x167ca18;
        const HEAD_D_VA: u32 = 0x167ca1c;
        const FIRST_FLAG_VA: u32 = 0x1670d29;
        const END_VA: u32 = 0x167ca39;
        const ENTRY_LEN: u32 = 0x2c;
        const KEPT_BITS: u8 = 0xef;
        *lf_checker_rt::global::<u32>(HEAD_A_VA) = 0;
        *lf_checker_rt::global::<u32>(HEAD_B_VA) = 0;
        for i in 0..4u32 {
            *lf_checker_rt::global::<u8>(HEAD_C_VA + i) = 0;
        }
        let mut cur = lf_checker_rt::relocated(FIRST_FLAG_VA);
        let end = lf_checker_rt::relocated(END_VA);
        while (cur as i32) < (end as i32) {
            let p = cur as *mut u8;
            p.write(p.read() & KEPT_BITS);
            cur += ENTRY_LEN;
        }
        *lf_checker_rt::global::<u32>(HEAD_D_VA) = 0;
        cur
    }
});
