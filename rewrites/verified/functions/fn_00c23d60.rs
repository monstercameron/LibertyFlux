// original: 0x00c23d60 stream_grid_lookup (proposed)
/// Look up `key` (signed against 0xe0f). Small keys derive grid coordinates
/// from the key by signed division with the global stride and forward them
/// with the global scales to the grid worker, returning the grid base.
/// Large keys scan the row table at 0x016b9c30 (`count` rows from
/// 0x016b9d70, 0x14 bytes each, signed): the row whose word at +0xc equals
/// the key yields the word at +0x10, else 0.
///
/// Original: 0x00c23d60 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00c23d60(key: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 0x010482b0;
        const SCALE_A: u32 = 0x010482ac;
        const SCALE_B: u32 = 0x010482b4;
        const COUNT: u32 = 0x016b9d70;
        const TABLE: u32 = 0x016b9c30;
        const GRID_TAG: u32 = 0x00ec5640;
        const GRID_BASE: u32 = 0x016c85c8;
        const SMALL_MAX: i32 = 0xe0f;
        if (key as i32) > SMALL_MAX {
            let n = (lf_checker_rt::relocated(COUNT) as *const i32).read_unaligned();
            if n <= 0 {
                return 0;
            }
            let mut p = lf_checker_rt::relocated(TABLE);
            let mut i: i32 = 0;
            loop {
                if (p.wrapping_add(0x0c) as *const u32).read_unaligned() == key {
                    return (p.wrapping_add(0x10) as *const u32).read_unaligned();
                }
                i = i.wrapping_add(1);
                p = p.wrapping_add(0x14);
                if !(i < n) {
                    return 0;
                }
            }
        } else {
            let r = lf_checker_rt::relocated;
            let stride = (r(STRIDE) as *const i32).read_unaligned();
            let q = (key as i32) / stride;
            let a = (r(SCALE_A) as *const i32).read_unaligned();
            let t0 = a.wrapping_mul(q);
            let b = (r(SCALE_B) as *const i32).read_unaligned();
            let t1 = (key as i32).wrapping_sub(b.wrapping_mul(q)).wrapping_mul(a);
            lf_checker_rt::callee_cdecl!(
                1, u32,
                r(GRID_BASE), r(GRID_TAG), a as u32, a as u32, t1 as u32, t0 as u32
            );
            r(GRID_BASE)
        }
    }
});
