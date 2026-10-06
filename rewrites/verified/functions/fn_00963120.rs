// original: 0x00963120 pool_entry_find_mark
/// Find a pool-table entry by key and address sum, then mark it.
///
/// Arguments are `(key, base, add)`. Scans the `0x400` 20-byte entries at
/// `0x120F2B8` for the first whose pointer is non-null, whose word at `+8`
/// equals `key` and whose word at `+0xC` equals `base + add` (wrapping);
/// the hit gets its flag byte at `+0x10` set to 1. Returns the hit index
/// times 5, or `0x13FB` (`0x3FF * 5`) when nothing matches.
lf_checker_rt::export!(cdecl, rw_00963120(key: u32, base: u32, add: u32) -> u32 {
    unsafe {
        const TAB: u32 = 0x120f2b8;
        const ENTRIES: u32 = 0x400;
        const STRIDE: u32 = 20;
        let target = base.wrapping_add(add);
        let mut dx = 0u32;
        let mut eax = 0u32;
        loop {
            let idx = (dx & 0xffff) as u16 as i16 as i32 as u32;
            eax = idx.wrapping_mul(5);
            let e = lf_checker_rt::relocated(TAB).wrapping_add(eax.wrapping_mul(4));
            if (e as *const u32).read_unaligned() != 0
                && (e.wrapping_add(8) as *const u32).read_unaligned() == key
                && (e.wrapping_add(0x0c) as *const u32).read_unaligned() == target
            {
                (e.wrapping_add(0x10) as *mut u8).write(1);
                return eax;
            }
            dx += 1;
            if !(((dx & 0xffff) as u16 as i16) < (ENTRIES as u16 as i16)) {
                break;
            }
        }
        eax
    }
});
