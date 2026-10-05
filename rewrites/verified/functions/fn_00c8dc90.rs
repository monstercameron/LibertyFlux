// original: 0x00c8dc90 find_nearest_in_radius (proposed)

/// Find the entity nearest a query point within a fixed radius.
///
/// `q` points to the query point (3 floats). The search covers the buckets
/// `TABLE[0..COUNT]` (a global pointer and a global 16-bit count); each
/// bucket whose box overlaps the query box `[q-R, q+R]` (`R` a global
/// radius) contributes a linked list of entities hung off `+0x20` and
/// chained through `+0x1c`.
///
/// An entity is considered when: the liveness probe (callee 1, cdecl)
/// answers 0; unless `flags & 0xff == 0`, its low 3 bits are non-zero and
/// its word at `+0x14` is 0; the position fetch (callee 2, thiscall) and
/// the readiness probe (callee 3, thiscall, must answer 0) run, bit 10 of
/// its first word is clear, `typefilter` is 0 or equals its low 3 bits,
/// and `match10` is 0 or equals its word at `+0x10`.
///
/// A considered entity scores when its vertical gap to the query point is
/// below `VSTEP (4.0)` and its distance below `R`; when `ctx` is non-null
/// the visibility probe (callee 4, cdecl) also runs, and mode 2 requires
/// it to answer non-zero. The winner is the lowest score, where mode 1
/// with a zero visibility answer is penalised to `score*2+5`; initial best
/// is the global `BEST_INIT`. Returns the winning entity or 0.
///
/// Float comparisons: every branch is an ordered comparison matching
/// the original's `comiss`+conditional-jump selection (`ja` is `>`,
/// `jbe` is `!(>)`); the absolute value keeps a NaN's exact bits by
/// flipping the sign only when `0.0 > dz`.
///
/// Original: 0x00c8dc90 (cdecl, six stack words). Returns a pointer or 0.
lf_checker_rt::export!(cdecl, rw_00c8dc90(q: u32, ctx: u32, mode: u32, typefilter: u32, match10: u32, flags: u32) -> u32 {
    unsafe {
        const RADIUS: u32 = 0x01050a88;
        const COUNT: u32 = 0x01713aa0;
        const TABLE: u32 = 0x01713a9c;
        const BEST_INIT: u32 = 0x00e9bd14;
        const VSTEP: u32 = 0x00fe8ab8;
        const TWO: u32 = 0x00fe8a24;
        const FIVE: u32 = 0x00fe8ad8;
        const SIGN_BIT: u32 = 0x8000_0000;
        const BK_MIN: u32 = 0x00;
        const BK_MAX: u32 = 0x10;
        const BK_HEAD: u32 = 0x20;
        const EN_NEXT: u32 = 0x1c;
        const EN_ACTIVE: u32 = 0x14;
        const EN_MATCH: u32 = 0x10;

        #[inline(always)]
        unsafe fn rd(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        let r: f32 = rd(lf_checker_rt::relocated(RADIUS));
        let (qx, qy, qz) = (rd(q), rd(q + 4), rd(q + 8));
        let (lox, loy, loz) = (sub(qx, r), sub(qy, r), sub(qz, r));
        let (hix, hiy, hiz) = (add(qx, r), add(qy, r), add(qz, r));
        let mut best: f32 = rd(lf_checker_rt::relocated(BEST_INIT));
        let mut winner: u32 = 0;
        let mut i: u32 = 0;
        // One out-slot for the whole scan, like the original's single frame
        // slot: leftovers from one entity are still there for the next.
        let mut pos = [0u32; 3];
        loop {
            let count = ((lf_checker_rt::relocated(COUNT) as *const u16).read_unaligned()) as u32;
            if i >= count {
                break;
            }
            let table = rd32(lf_checker_rt::relocated(TABLE));
            let bucket = rd32(table.wrapping_add(i.wrapping_mul(4)));
            let overlaps = !(rd(bucket + BK_MIN) > hix)
                && !(rd(bucket + BK_MIN + 4) > hiy)
                && !(rd(bucket + BK_MIN + 8) > hiz)
                && !(lox > rd(bucket + BK_MAX))
                && !(loy > rd(bucket + BK_MAX + 4))
                && !(loz > rd(bucket + BK_MAX + 8));
            let mut ent = if overlaps { rd32(bucket + BK_HEAD) } else { 0 };
            while ent != 0 {
                let mut vis = 0u32;
                'entity: {
                    if (lf_checker_rt::callee_cdecl!(1u32, u32, ent) & 0xff) != 0 {
                        break 'entity;
                    }
                    if (flags & 0xff) != 0 {
                        if rd32(ent) & 7 == 0 || rd32(ent + EN_ACTIVE) != 0 {
                            break 'entity;
                        }
                    }
                    let out = &mut pos as *mut [u32; 3] as u32;
                    lf_checker_rt::callee_thiscall!(2u32, u32, ent, out, 0u32);
                    if (lf_checker_rt::callee_thiscall!(3u32, u32, ent) & 0xff) != 0 {
                        break 'entity;
                    }
                    let head = rd32(ent);
                    if (head >> 10) & 1 != 0 {
                        break 'entity;
                    }
                    if typefilter != 0 && (head & 7) != typefilter {
                        break 'entity;
                    }
                    if match10 != 0 && rd32(ent + EN_MATCH) != match10 {
                        break 'entity;
                    }
                    let (px, py, pz) = (
                        f32::from_bits(pos[0]),
                        f32::from_bits(pos[1]),
                        f32::from_bits(pos[2]),
                    );
                    let dz = sub(qz, pz);
                    // fabs that preserves an exact NaN: flip only below zero.
                    let az = if 0.0f32 > dz {
                        f32::from_bits(dz.to_bits() ^ SIGN_BIT)
                    } else {
                        dz
                    };
                    if !(rd(lf_checker_rt::relocated(VSTEP)) > az) {
                        break 'entity;
                    }
                    let dx = sub(qx, px);
                    let dy = sub(qy, py);
                    let d2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
                    // sqrtps low lane; sqrtss computes the same bits.
                    let d = core::hint::black_box(d2).sqrt();
                    if !(r > d) {
                        break 'entity;
                    }
                    if ctx != 0 {
                        vis = lf_checker_rt::callee_cdecl!(4u32, u32, ent, ctx, out);
                        if mode == 2 && (vis & 0xff) == 0 {
                            break 'entity;
                        }
                    }
                    // Penalised only on the ctx path with mode 1 and a zero answer.
                    let mut score = d;
                    if ctx != 0 && mode == 1 && (vis & 0xff) == 0 {
                        let two: f32 = rd(lf_checker_rt::relocated(TWO));
                        let five: f32 = rd(lf_checker_rt::relocated(FIVE));
                        score = add(mul(score, two), five);
                    }
                    if !(best > score) {
                        break 'entity;
                    }
                    best = score;
                    winner = ent;
                }
                ent = rd32(ent + EN_NEXT);
            }
            i = i.wrapping_add(1);
        }
        winner
    }
});
