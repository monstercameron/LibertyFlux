// original: 0x00a97860 fade_vec_find_or_append
/// Finds a color record within tolerance or appends it to the table.
///
/// Scans the table for a record whose first three words all fall within
/// 0.01 of the query (compared as absolute float differences); an exact-enough
/// hit returns its index, otherwise the query (all four words) is appended,
/// the count is bumped and the new index is returned.
export!(stdcall, rw_00a97860(base: u32, countp: u32, query: u32) -> u32 {
    unsafe {
        let eps = *(global::<f32>(0xFE870C) as *const f32);
        let count = *(countp as *const i32);
        let mut i: i32 = 0;
        while i < count {
            let e = base.wrapping_add((i as u32).wrapping_mul(16));
            let d0 = (*(e as *const f32) - *(query as *const f32)).abs();
            if eps > d0 {
                let d1 = (*((e + 4) as *const f32) - *((query + 4) as *const f32)).abs();
                if eps > d1 {
                    let d2 = (*((e + 8) as *const f32) - *((query + 8) as *const f32)).abs();
                    if eps > d2 {
                        return i as u32;
                    }
                }
            }
            i += 1;
        }
        let e = base.wrapping_add((count as u32).wrapping_mul(16));
        *(e as *mut u32) = *(query as *const u32);
        *((e + 4) as *mut u32) = *((query + 4) as *const u32);
        *((e + 8) as *mut u32) = *((query + 8) as *const u32);
        *((e + 12) as *mut u32) = *((query + 12) as *const u32);
        *(countp as *mut u32) = (count as u32).wrapping_add(1);
        count as u32
    }
});
