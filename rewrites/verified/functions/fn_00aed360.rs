// original: 0x00AED360 kv_partition_step (proposed)

/// One Hoare partition sweep over 8-byte entries, pivot given by value.
///
/// `lo` and `hi` delimit the range, `pivot` is the pivot key (not
/// necessarily present). A cursor runs up from `lo` past keys below the
/// pivot while another runs down from `hi - 8` past keys above it (both
/// unsigned); when they have not crossed, the two pairs are exchanged and
/// the sweep continues. Returns the crossing point. There are no bound
/// checks, so callers keep a key at or above the pivot at the top and a
/// key at or below it at the bottom.
///
/// Original: 0x00AED360 (cdecl, three stack words, no calls).
lf_checker_rt::export!(cdecl, rw_00aed360(lo: u32, hi: u32, pivot: u32) -> u32 {
    unsafe {
        const ENTRY: u32 = 8;
        const KEY_OFF: u32 = 0;
        const VAL_OFF: u32 = 4;
        let mut up = lo;
        let mut down = hi;
        loop {
            while ((up.wrapping_add(KEY_OFF)) as *const u32).read_unaligned() < pivot {
                up = up.wrapping_add(ENTRY);
            }
            down = down.wrapping_sub(ENTRY);
            while pivot < ((down.wrapping_add(KEY_OFF)) as *const u32).read_unaligned() {
                down = down.wrapping_sub(ENTRY);
            }
            if up >= down {
                return up;
            }
            let ku = ((up.wrapping_add(KEY_OFF)) as *const u32).read_unaligned();
            let vu = ((up.wrapping_add(VAL_OFF)) as *const u32).read_unaligned();
            let kd = ((down.wrapping_add(KEY_OFF)) as *const u32).read_unaligned();
            let vd = ((down.wrapping_add(VAL_OFF)) as *const u32).read_unaligned();
            ((up.wrapping_add(KEY_OFF)) as *mut u32).write_unaligned(kd);
            ((up.wrapping_add(VAL_OFF)) as *mut u32).write_unaligned(vd);
            ((down.wrapping_add(KEY_OFF)) as *mut u32).write_unaligned(ku);
            ((down.wrapping_add(VAL_OFF)) as *mut u32).write_unaligned(vu);
            up = up.wrapping_add(ENTRY);
        }
    }
});
