// original: 0x005D33E0 pool_get_at_or_null

/// Return the pool item's address unless the selected flag byte is marked
/// unavailable.
///
/// The pool stores its item base at offset 0, the byte flags pointer at 4,
/// and the item stride at 12. The sole stack argument is an unsigned byte
/// index. Flag bit 7 returns null; otherwise the result is the 32-bit wrapped
/// sum of the item base and the unsigned wrapped product of index and stride.
/// The thiscall callee removes the four-byte index argument.
lf_checker_rt::export!(thiscall, rw_005d33e0(this: u32, index: u32) -> u32 {
    unsafe {
        const ITEM_BASE: u32 = 0;
        const FLAGS_POINTER: u32 = 4;
        const ITEM_STRIDE: u32 = 12;
        const UNAVAILABLE: u8 = 0x80;

        let pool = this as *const u32;
        let item_base = pool.add((ITEM_BASE / 4) as usize).read_unaligned();
        let flags = pool.add((FLAGS_POINTER / 4) as usize).read_unaligned();
        let stride = pool.add((ITEM_STRIDE / 4) as usize).read_unaligned();
        if ((flags.wrapping_add(index)) as *const u8).read_unaligned() & UNAVAILABLE != 0 {
            0
        } else {
            item_base.wrapping_add(index.wrapping_mul(stride))
        }
    }
});
