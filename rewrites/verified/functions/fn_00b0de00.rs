// original: 0x00b0de00 net_find_entry_near_point (proposed)

/// Scan the network node table for an entry near a queried point.
///
/// `out` points to 16 writable bytes. The function first asks callee 1 for a
/// threshold float (constant argument `QUERY_ARG`, x87 single return); when
/// that value is above `THRESH_LIMIT` (+0.0, so NaN proceeds) it returns 0
/// without touching `out`. Otherwise it asks callee 2 to fill four floats at
/// a stack slot (the query point `q0..q2` plus a carried fourth word) and
/// walks the node table from `ROW_FIRST` up to `ROW_END` in `ROW_STRIDE`
/// steps (1512 rows). A row matches when its flag byte at `+0x28` is
/// non-zero, its sign-extended 16-bit id at `+0x24` equals either of the two
/// filter words in static storage, and its link word at `-0x18` is non-zero.
/// For a matching row the distance from the query point to the row's three
/// floats at `-4`, `+0`, `+4` is formed exactly as the original does:
/// `((dy*dy + dx*dx) + dz*dz)` with the subtractions in x, y, z order, then
/// a square root. The first row whose distance is below `RADIUS` (8.0, so a
/// NaN distance never matches) wins: the row's three floats go to
/// `out[0..3]`, the query's fourth word to `out[3]`, and the function
/// returns 1. When no row matches it returns 0 and leaves `out` untouched.
///
/// Original: 0x00b0de00 (cdecl, one stack word, `al` return).
lf_checker_rt::export!(cdecl, rw_00b0de00(out: u32) -> u8 {
    unsafe {
        const QUERY_ARG: u32 = 0x169;
        const THRESH_LIMIT: f32 = f32::from_bits(0x0000_0000);
        const RADIUS: f32 = f32::from_bits(0x4100_0000);
        const FILTER_A: u32 = 0x012F_9DA4;
        const FILTER_B: u32 = 0x012F_9DB0;
        const ROW_FIRST: u32 = 0x0161_567C;
        const ROW_END: u32 = 0x0163_2B3C;
        const ROW_STRIDE: u32 = 0x50;
        const OFF_FLAG: u32 = 0x28;
        const OFF_ID: u32 = 0x24;
        const QUERY_CALLEE: u32 = 1;
        const POINT_CALLEE: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> i32 {
            unsafe { (a as *const i16).read_unaligned() as i32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        let thresh: f32 = lf_checker_rt::callee_cdecl!(QUERY_CALLEE, f32, QUERY_ARG);
        if thresh > THRESH_LIMIT {
            return 0;
        }
        let mut q = [0f32; 4];
        lf_checker_rt::callee_cdecl!(
            POINT_CALLEE,
            u32,
            q.as_mut_ptr() as u32
        );
        let filter_a = rd32(lf_checker_rt::relocated(FILTER_A));
        let filter_b = rd32(lf_checker_rt::relocated(FILTER_B));
        let filter_a = filter_a as i32;
        let filter_b = filter_b as i32;
        let mut row = lf_checker_rt::relocated(ROW_FIRST);
        let end = lf_checker_rt::relocated(ROW_END);
        loop {
            if rd8(row + OFF_FLAG) != 0 {
                let id = rd16(row + OFF_ID);
                if id == filter_a || id == filter_b {
                    // The link word sits 0x18 bytes below the row cursor.
                    if rd32(row.wrapping_sub(0x18)) != 0 {
                        let f0 = rdf(row.wrapping_sub(4));
                        let f1 = rdf(row);
                        let f2 = rdf(row + 4);
                        let dx = sub(q[0], f0);
                        let dy = sub(q[1], f1);
                        let dz = sub(q[2], f2);
                        let dist = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz)).sqrt();
                        if RADIUS > dist {
                            (out as *mut u32).write_unaligned(f0.to_bits());
                            ((out + 4) as *mut u32).write_unaligned(f1.to_bits());
                            ((out + 8) as *mut u32).write_unaligned(f2.to_bits());
                            ((out + 12) as *mut u32).write_unaligned(q[3].to_bits());
                            return 1;
                        }
                    }
                }
            }
            row = row.wrapping_add(ROW_STRIDE);
            if row >= end {
                return 0;
            }
        }
    }
});
