// original: 0x00b42570 bitset_alloc_next_clear
/// Allocate the next clear bit of the global bitset, wrapping at the limit.
///
/// Scans forward from the stored cursor (wrapping to 0 past the limit) for
/// at most `limit` positions; on a clear bit stores the following position
/// as the new cursor and returns the found index, else returns 0xFFFF.
export!(cdecl, rw_b42570() -> u32 {
    unsafe {
        let cursor = global::<u32>(0x16B8F94);
        let limit = (global::<u32>(0xEEDE18)).read();
        let mut remaining = limit;
        if remaining == 0 {
            return 0xFFFF;
        }
        let bits = (global::<u32>(0x16B9DB8)).read();
        let mut pos = cursor.read();
        loop {
            let mask = 1u32.wrapping_shl(pos & 0x1f);
            let idx = (pos as i32).wrapping_shr(5) as u32;
            let word = (bits.wrapping_add(idx.wrapping_mul(4)) as *const u32).read();
            if word & mask == 0 {
                let next = pos.wrapping_add(1);
                cursor.write(if next < limit { next } else { 0 });
                return pos;
            }
            pos = pos.wrapping_add(1);
            if pos >= limit {
                pos = 0;
            }
            remaining = remaining.wrapping_sub(1);
            if remaining == 0 {
                return 0xFFFF;
            }
        }
    }
});
