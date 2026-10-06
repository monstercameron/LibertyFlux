// original: 0x00891400 audsound_slot_find_and_reindex
/// Finds `target` among the eight slot pointers, then re-derives the slot.
///
/// Each of the eight bytes at `this+0x48` (in order) resolves to a pointer:
/// 0xff means null, otherwise `stride * byte + table[idx * 0x6f40 + 0x6f10]`
/// with `stride` and `table` from the globals and `idx` the byte at
/// `this+0x40` (all arithmetic wrapping unsigned). The scan stops at the
/// first slot whose pointer equals `target`. When none matches, the last
/// slot's pointer is returned and nothing is written. On a match at slot `i`:
/// when `value` is zero the slot byte is set to 0xff and 0xff returned,
/// otherwise the slot byte becomes the low 8 bits of the UNSIGNED quotient
/// `(value - table[idx * 0x6f40 + 0x6f10]) / stride` (the dividend's high
/// half is zero, so the quotient always fits; `stride` is never zero) and
/// that byte is returned zero-extended. Indices and the bound compare
/// unsigned. Original: 0x00891400 (thiscall, two stack words: target, value).
export!(thiscall, rw_00891400(this: *mut u8, target: u32, value: u32) -> u32 {
    unsafe {
        const SLOTS: usize = 0x48;
        const NSLOTS: u32 = 8;
        const INDEX: usize = 0x40;
        const ROW: u32 = 0x6f40;
        const ROW_OFF: u32 = 0x6f10;
        const EMPTY: u8 = 0xff;
        const STRIDE_G: u32 = 0x115d964;
        const TABLE_G: u32 = 0x115d988;
        let stride = *global::<u32>(STRIDE_G);
        let table = *global::<u32>(TABLE_G);
        let idx = *this.add(INDEX) as u32;
        let base = *((table.wrapping_add(idx.wrapping_mul(ROW)).wrapping_add(ROW_OFF)) as *const u32);
        let mut i = 0u32;
        loop {
            let b = *this.add(SLOTS + i as usize);
            let lookup = if b == EMPTY {
                0
            } else {
                stride.wrapping_mul(b as u32).wrapping_add(base)
            };
            if lookup == target {
                if value == 0 {
                    *this.add(SLOTS + i as usize) = EMPTY;
                    return EMPTY as u32;
                }
                let q = value.wrapping_sub(base) / stride;
                let al = q as u8;
                *this.add(SLOTS + i as usize) = al;
                return al as u32;
            }
            i = i.wrapping_add(1);
            if i >= NSLOTS {
                return lookup;
            }
        }
    }
});
