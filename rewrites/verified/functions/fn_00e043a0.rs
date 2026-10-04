// original: 0x00e043a0 find_record_by_field
/// Finds the first 12-byte record whose word at offset 4 equals the needle.
///
/// Scans `count` records (count read from 0xF0DAA0) starting at `base`,
/// stepping 12 bytes; returns a pointer to the first in-range match, or
/// null when no in-range record matches.
export!(cdecl, rw_00e043a0(needle: u32, base: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 12;
        let count = *global::<u32>(0xF0DAA0);
        let mut cur = base;
        loop {
            if *((cur.wrapping_add(4)) as *const u32) == needle {
                break;
            }
            let end = base.wrapping_add(count.wrapping_mul(STRIDE));
            cur = cur.wrapping_add(STRIDE);
            if cur >= end {
                break;
            }
        }
        let end = base.wrapping_add(count.wrapping_mul(STRIDE));
        if cur >= end {
            return 0;
        }
        if *((cur.wrapping_add(4)) as *const u32) != needle {
            return 0;
        }
        cur
    }
});
