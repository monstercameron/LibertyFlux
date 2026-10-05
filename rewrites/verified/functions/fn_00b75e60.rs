// original: 0x00b75e60 pool_entry_revoke (proposed)

/// Revoke pool entries tagged with the low 16 bits of `key`.
///
/// Scans one word per 44-byte entry from 0x1670D20 up to (not including)
/// 0x167CA30: an entry whose tag word equals the key's low half, whose flag
/// byte at +9 has bit 4 set and whose next byte has bit 4 clear, has the
/// flag bit cleared and its tag zeroed, and the live count at 0x167CA10 is
/// decremented per revocation. Returns the final cursor (the end address).
///
/// Original: 0x00b75e60 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00b75e60(key: u32) -> u32 {
    unsafe {
        const COUNT_VA: u32 = 0x167ca10;
        const FIRST_FLAG_VA: u32 = 0x1670d29;
        const END_VA: u32 = 0x167ca39;
        const ENTRY_LEN: u32 = 0x2c;
        const TAG_BACK: u32 = 9;
        const ARMED_BIT: u8 = 0x10;
        const KEPT_BITS: u8 = 0xef;
        let tag = key as u16;
        let mut live = *lf_checker_rt::global::<u32>(COUNT_VA);
        let mut cur = lf_checker_rt::relocated(FIRST_FLAG_VA);
        let end = lf_checker_rt::relocated(END_VA);
        while (cur as i32) < (end as i32) {
            let at = cur - TAG_BACK;
            if ((at as *const u16).read_unaligned() == tag)
                && ((cur as *const u8).read() & ARMED_BIT != 0)
                && (((cur + 1) as *const u8).read() & ARMED_BIT == 0)
            {
                let p = cur as *mut u8;
                p.write(p.read() & KEPT_BITS);
                (at as *mut u16).write_unaligned(0);
                live = live.wrapping_sub(1);
            }
            cur += ENTRY_LEN;
        }
        *lf_checker_rt::global::<u32>(COUNT_VA) = live;
        cur
    }
});
