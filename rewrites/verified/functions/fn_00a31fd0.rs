// original: 0x00a31fd0 pool_iterator_18b6f1c
/// Pool cursor step over the pool at its own global record; same walk as
/// `rw_00a31e40`: restart from the count on a negative cursor, skip
/// 0x80-flagged slots downward, return the first live slot or null.
export!(thiscall, rw_00a31fd0(cursor: *mut i32) -> u32 {
    unsafe {
        let pool = *global::<*const u32>(0x18B6F1C);
        if *cursor <= -1 {
            *cursor = *(pool.add(2) as *const i32);
        }
        let mut idx = (*cursor).wrapping_sub(1);
        if idx < 0 {
            *cursor = -1;
            return 0;
        }
        let flags = *(pool.add(1) as *const *const u8);
        let base = *pool;
        let stride = *(pool.add(3)) as i32;
        loop {
            if *flags.offset(idx as isize) & 0x80 == 0 {
                let slot = base.wrapping_add(stride.wrapping_mul(idx) as u32);
                if slot != 0 {
                    *cursor = idx;
                    return slot;
                }
            }
            idx = idx.wrapping_sub(1);
            if idx < 0 {
                *cursor = -1;
                return 0;
            }
        }
    }
});
