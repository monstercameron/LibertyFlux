// original: 0x00900c30 input_compact_entries (proposed)
/// Compact 8-byte entries: move the tail starting at `src` down to `dst`.
///
/// `this` points to the list header (word `+0` the entry base, 16-bit word
/// `+4` the entry count). Eight-byte entries are copied forward from `src`
/// up to `base + count * 8`, landing at `dst`; the count is then reduced by
/// `(src - dst) / 8`, dropping the entries before `src`. Returns `dst`.
/// Thiscall with two stack words, callee cleanup.
export!(thiscall, rw_00900c30(this: u32, dst: u32, src: u32) -> u32 {
    unsafe {
        const BASE_OFF: u32 = 0x00;
        const COUNT_OFF: u32 = 0x04;
        const ENTRY: u32 = 8;
        let base = ((this.wrapping_add(BASE_OFF)) as *const u32).read_unaligned();
        let count = ((this.wrapping_add(COUNT_OFF)) as *const u16).read_unaligned() as u32;
        let end = base.wrapping_add(count.wrapping_mul(ENTRY));
        let mut cur = src;
        while cur != end {
            let lo = (cur as *const u32).read_unaligned();
            (dst.wrapping_add(cur.wrapping_sub(src)) as *mut u32).write_unaligned(lo);
            let hi = ((cur.wrapping_add(4)) as *const u32).read_unaligned();
            ((dst.wrapping_add(cur.wrapping_sub(src)).wrapping_add(4)) as *mut u32)
                .write_unaligned(hi);
            cur = cur.wrapping_add(ENTRY);
        }
        let dropped = (src.wrapping_sub(dst) >> 3) as u16;
        let left = (count as u16).wrapping_sub(dropped);
        ((this.wrapping_add(COUNT_OFF)) as *mut u16).write_unaligned(left);
        dst
    }
});
