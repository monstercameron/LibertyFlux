// original: 0x00a31e40 pool_iterator_16f7d60
/// Pool cursor step: walks the cursor (`*this`, an entity-index word owned
/// by the caller) down through one object pool, skipping slots whose flag
/// byte has bit 0x80 set, and returns the first live slot address, or null
/// when no live slot remains below the cursor. A negative cursor restarts
/// the walk from the pool count. Each pool is a global record of four
/// dwords: entry base, flag bytes, live count, entry stride.
export!(thiscall, rw_00a31e40(cursor: *mut i32) -> u32 {
    unsafe {
        let pool = *global::<*const u32>(0x16F7D60);
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
