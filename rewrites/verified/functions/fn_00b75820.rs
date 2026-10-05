// original: 0x00b75820 pool_table_reset (proposed)

/// Reset the pool table: clear two header words, reset the slots, clear a
/// flag bit down the table.
///
/// Zeroes the words at 0x167ca10 and 0x167ca14, calls the slot-reset callee
/// (thiscall on 0x167ca20; its answer is ignored), then clears bit 4 of one
/// byte per 44-byte entry from 0x1670d29 up to (not including) 0x167ca39.
/// Returns the final cursor (the end address).
///
/// Original: 0x00b75820 (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_00b75820() -> u32 {
    unsafe {
        const HEAD_A_VA: u32 = 0x167ca10;
        const HEAD_B_VA: u32 = 0x167ca14;
        const SLOTS_VA: u32 = 0x167ca20;
        const FIRST_FLAG_VA: u32 = 0x1670d29;
        const END_VA: u32 = 0x167ca39;
        const ENTRY_LEN: u32 = 0x2c;
        const KEPT_BITS: u8 = 0xef;
        const SLOT_RESET: u32 = 1;
        *lf_checker_rt::global::<u32>(HEAD_A_VA) = 0;
        *lf_checker_rt::global::<u32>(HEAD_B_VA) = 0;
        let slots = lf_checker_rt::relocated(SLOTS_VA);
        let _: u32 = lf_checker_rt::callee_thiscall!(SLOT_RESET, u32, slots);
        let mut cur = lf_checker_rt::relocated(FIRST_FLAG_VA);
        let end = lf_checker_rt::relocated(END_VA);
        while (cur as i32) < (end as i32) {
            let p = cur as *mut u8;
            p.write(p.read() & KEPT_BITS);
            cur += ENTRY_LEN;
        }
        cur
    }
});
