// original: 0x00c8d9b0 find_best_in_shell (proposed)

/// Find the entity in a distance shell that best faces a reference point.
///
/// `q` points to the query point (3 floats); the search covers the buckets
/// `TABLE[0..COUNT]` exactly as in `rw_00c8dc90`, with the query box
/// `[q-rmax, q+rmax]` and the same entity chain layout (`+0x20`/`+0x1c`).
///
/// An entity is considered when: the liveness probe (callee 1, cdecl)
/// answers 0; its low 3 bits are non-zero and its word at `+0x14` is 0;
/// the position fetch (callee 2, thiscall) and the readiness probe
/// (callee 3, thiscall, must answer 0) run; bit 10 of its first word is
/// clear; it is not one of the `excl_n` pointers in `excl` (a non-positive
/// count skips the scan); and the callback `cb` (when non-null, cdecl with
/// the entity, the fetched position and `cbarg`) answers non-zero.
///
/// A considered entity scores when the visibility probe (callee 4, cdecl
/// with the entity, `ctx` and the fetched position) has run, `d2` (the
/// squared distance from `q` to the fetched position, summed `y+x` then
/// `+z`) lies in `[rmin*rmin, rmax*rmax)`, and the squared distance from
/// the fetched position to `ctx` is not below `d2`; mode 2 additionally
/// requires a non-zero visibility answer. The winner is the lowest
/// `sqrt(d2)`, penalised to `sqrt*2+5` for mode 1 with a zero visibility
/// answer; initial best is the global `BEST_INIT`. A winner is reported
/// through callee 6 (thiscall with the winner and `a0`) and returned.
///
/// Ordering notes: the shell test keeps `d2` only when `d2 >= rmin*rmin`
/// (below-or-unordered exits) and `rmax*rmax > d2`; the facing test exits
/// when `sqrt(d2)*sqrt(d2)` is strictly above the second squared distance.
/// All comparisons are ordered, matching `comiss` semantics.
///
/// Original: 0x00c8d9b0 (cdecl, eleven stack words; the word at `+0x14` is
/// never read). Returns a pointer or 0.
lf_checker_rt::export!(cdecl, rw_00c8d9b0(a0: u32, q: u32, ctx: u32, _pad: u32, excl: u32, excl_n: u32, mode: u32, rmin: u32, rmax: u32, cb: u32, cbarg: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x01713aa0;
        const TABLE: u32 = 0x01713a9c;
        const BEST_INIT: u32 = 0x00e9bd14;
        const TWO: u32 = 0x00fe8a24;
        const FIVE: u32 = 0x00fe8ad8;
        const BK_MIN: u32 = 0x00;
        const BK_MAX: u32 = 0x10;
        const BK_HEAD: u32 = 0x20;
        const EN_NEXT: u32 = 0x1c;
        const EN_ACTIVE: u32 = 0x14;

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

        let rmax = f32::from_bits(rmax);
        let rmin = f32::from_bits(rmin);
        let (qx, qy, qz) = (rd(q), rd(q + 4), rd(q + 8));
        let (lox, loy, loz) = (sub(qx, rmax), sub(qy, rmax), sub(qz, rmax));
        let (hix, hiy, hiz) = (add(qx, rmax), add(qy, rmax), add(qz, rmax));
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
                    let head0 = rd32(ent);
                    if head0 & 7 == 0 || rd32(ent + EN_ACTIVE) != 0 {
                        break 'entity;
                    }
                    let out = &mut pos as *mut [u32; 3] as u32;
                    lf_checker_rt::callee_thiscall!(2u32, u32, ent, out, 0u32);
                    if (lf_checker_rt::callee_thiscall!(3u32, u32, ent) & 0xff) != 0 {
                        break 'entity;
                    }
                    if (rd32(ent) >> 10) & 1 != 0 {
                        break 'entity;
                    }
                    // Exclusion scan; a non-positive count skips it.
                    // (The original compares signed: `(an instruction of the original); jle`.)
                    if (excl_n as i32) > 0 {
                        let mut k: u32 = 0;
                        loop {
                            if rd32(excl.wrapping_add(k.wrapping_mul(4))) == ent {
                                break 'entity;
                            }
                            k = k.wrapping_add(1);
                            if !(k < excl_n) {
                                break;
                            }
                        }
                    }
                    if cb != 0 {
                        let f: extern "cdecl" fn(u32, u32, u32) -> u32 =
                            unsafe { core::mem::transmute(cb as usize) };
                        if (f(ent, out, cbarg) & 0xff) == 0 {
                            break 'entity;
                        }
                    }
                    vis = lf_checker_rt::callee_cdecl!(4u32, u32, ent, ctx, out);
                    if mode == 2 && (vis & 0xff) == 0 {
                        break 'entity;
                    }
                    let (px, py, pz) = (
                        f32::from_bits(pos[0]),
                        f32::from_bits(pos[1]),
                        f32::from_bits(pos[2]),
                    );
                    let dx = sub(qx, px);
                    let dy = sub(qy, py);
                    let dz = sub(qz, pz);
                    let d2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
                    // `comiss d2,rmin2; jb`: below-or-unordered exits.
                    if !(d2 >= mul(rmin, rmin)) {
                        break 'entity;
                    }
                    if !(mul(rmax, rmax) > d2) {
                        break 'entity;
                    }
                    let s = core::hint::black_box(d2).sqrt();
                    let (cx, cy, cz) = (rd(ctx), rd(ctx + 4), rd(ctx + 8));
                    let ex = sub(px, cx);
                    let ey = sub(py, cy);
                    let ez = sub(pz, cz);
                    let e2 = add(add(mul(ey, ey), mul(ex, ex)), mul(ez, ez));
                    if mul(s, s) > e2 {
                        break 'entity;
                    }
                    let mut score = s;
                    if mode == 1 && (vis & 0xff) == 0 {
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
        if winner != 0 {
            lf_checker_rt::callee_thiscall!(6u32, u32, winner, a0);
        }
        winner
    }
});
