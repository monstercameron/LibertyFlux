// original: 0x005e43f0 pool_find_matching_slot
// rw_005e43f0: find the topmost live pool slot whose probe accepts.
//
// Scans the script-object pool from slot `count - 1` down, skipping dead
// slots, and probes each live slot's entry with the match callback. The
// callback answers zero on a match; the first match wins and its index is
// returned, otherwise zero (which also covers the empty pool).
export!(stdcall, rw_005e43f0(arg: u32) -> u32 {
    unsafe {
        let mut n = *global::<u32>(POOL_COUNT);
        if n == 0 {
            return 0;
        }
        let bitmap = *global::<u32>(POOL_BITMAP) as *const u8;
        let stride = *global::<u32>(POOL_STRIDE);
        let base = *global::<u32>(POOL_BASE);
        loop {
            n = n.wrapping_sub(1);
            if *bitmap.add(n as usize) & SLOT_DEAD == 0 {
                let entry = base.wrapping_add(stride.wrapping_mul(n));
                if entry != 0 {
                    let probe: u32 = callee_cdecl!(1, u32, entry, arg, 0x20);
                    if probe == 0 {
                        return n;
                    }
                }
            }
            if n == 0 {
                return 0;
            }
        }
    }
});
