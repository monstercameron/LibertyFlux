// original: 0x0094BCD0 GroupHandleFromIndex (symbols)

/// Combine an index with a kind-selected table word into a group handle.
///
/// Only the LOW byte of `kind` is used, minus 4 as an UNSIGNED switch:
/// kinds 4, 8, 9, 10 read a word from their table (strides 4, 0x5C, 32, 48
/// times `idx`) and return (word << 16) | idx; every other kind (below 4,
/// 5..7, above 10) returns -1. No calls.
///
/// Original: 0x0094BCD0 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_0094BCD0(idx: u32, kind: u32) -> u32 {
    unsafe {
        const T1: u32 = 0x11E6252;
        const T2: u32 = 0x1666C8C;
        const T3: u32 = 0x11D95F2;
        const T4: u32 = 0x11D97F0;
        const MISS: u32 = 0xFFFF_FFFF;
        let k = (kind & 0xFF).wrapping_sub(4);
        if k > 6 {
            return MISS;
        }
        let w = match k {
            0 => (lf_checker_rt::relocated(T1).wrapping_add(idx.wrapping_mul(4))
                as *const u16)
                .read_unaligned() as u32,
            4 => (lf_checker_rt::relocated(T2).wrapping_add(idx.wrapping_mul(0x5C))
                as *const u16)
                .read_unaligned() as u32,
            5 => (lf_checker_rt::relocated(T3).wrapping_add(idx << 5) as *const u16)
                .read_unaligned() as u32,
            6 => (lf_checker_rt::relocated(T4).wrapping_add(idx.wrapping_mul(48))
                as *const u16)
                .read_unaligned() as u32,
            _ => return MISS,
        };
        (w << 16) | idx
    }
});
