// original: 0x00C8DF30 ped_task_best_in_radius (proposed)

/// Scan the ped-task spatial index for the entity nearest a point and return it.
///
/// `center` points to three floats (x, y, z). `radius` is the search radius.
/// The index is a small array of sector nodes: `SECTOR_TABLE` points at the
/// pointer array and `SECTOR_COUNT` (a 16-bit word) holds its length. Each
/// node carries an axis-aligned box (minimum corner at `+0x00`, maximum
/// corner at `+0x10`) and the head of an entity list at `+0x20`; entities
/// chain through `+0x1c`.
///
/// A node is skipped unless its box overlaps the query cube (`center`
/// plus/minus `radius` on every axis). Each entity of an overlapping node is
/// then filtered in order: callee 0 (cdecl, vetoes on a non-zero low byte),
/// the low three flag bits at `+0x00` (vetoes when all clear), the marker
/// dword at `+0x14` (vetoes when non-zero), callee 1 (thiscall, writes the
/// entity position into the scratch slot), callee 2 (thiscall, vetoes on a
/// non-zero low byte), the height window (the absolute z gap must be below
/// `Z_LIMIT`), and the sphere test (squared distance below `radius * radius`).
/// `filter_cb`, when non-null, is a cdecl callback `(entity, position,
/// user)` that vetoes on a zero low byte.
///
/// Scoring: `score_cb`, when non-null, is a cdecl callback of the same shape
/// returning a float score on the x87 stack, which replaces the distance for
/// the best-so-far comparison. Otherwise `extra` (when non-null) selects a
/// callee 5 (cdecl) probe whose answer combines with `mode`: mode 2 keeps the
/// candidate only on a non-zero answer, mode 1 scales the distance by
/// `MODE1_MUL` and adds `MODE1_ADD` on a zero answer, any other mode keeps
/// the plain distance. The candidate with the smallest score wins; ties keep
/// the earlier one. Returns the winning entity pointer, or null when no
/// candidate passed (or the index is empty).
///
/// All six box tests skip only on an ordered `above`, so an unordered
/// (NaN) comparison overlaps. The threshold tests are the complementary
/// `below-or-equal-or-unordered`. Float operation order matches the
/// original exactly (see the helpers); comparisons are written so NaN takes
/// the same branch on both sides.
///
/// Original: 0x00C8DF30 (cdecl, seven stack words).
lf_checker_rt::export!(cdecl, rw_00C8DF30(
    center: u32,
    radius: u32,
    extra: u32,
    mode: u32,
    score_cb: u32,
    filter_cb: u32,
    user: u32,
) -> u32 {
    unsafe {
        const SECTOR_TABLE: u32 = 0x1713A9C;
        const SECTOR_COUNT: u32 = 0x1713AA0;
        const BEST_INIT: u32 = 0xE9BD14;
        const Z_LIMIT: u32 = 0xFE8AB8;
        const MODE1_MUL: u32 = 0xFE8A24;
        const MODE1_ADD: u32 = 0xFE8AD8;
        const SIGN_MASK: u32 = 0x8000_0000;
        const NODE_MAX: u32 = 0x10;
        const NODE_HEAD: u32 = 0x20;
        const ENT_MARKER: u32 = 0x14;
        const ENT_NEXT: u32 = 0x1c;
        const FLAG_MASK: u8 = 7;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
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
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        /// `comiss p, q` followed by `ja`: taken only on ordered-above.
        #[inline(always)]
        fn above(p: f32, q: f32) -> bool {
            p > q
        }
        /// `comiss p, q` followed by `jbe`: taken on below, equal or unordered.
        #[inline(always)]
        fn below_or_unordered(p: f32, q: f32) -> bool {
            !(p > q)
        }
        #[inline(always)]
        fn abs_bits(v: f32) -> f32 {
            f32::from_bits(v.to_bits() ^ SIGN_MASK)
        }

        let cx = rdf(center);
        let cy = rdf(center.wrapping_add(4));
        let cz = rdf(center.wrapping_add(8));
        let rad = f32::from_bits(radius);
        let min_x = sub(cx, rad);
        let min_y = sub(cy, rad);
        let min_z = sub(cz, rad);
        let max_x = add(cx, rad);
        let max_y = add(cy, rad);
        let max_z = add(cz, rad);

        let mut best = rdf(lf_checker_rt::relocated(BEST_INIT));
        let mut best_ent = 0u32;
        // Scratch position slot, reused across candidates exactly like the
        // original's frame slot (a callee snapshot compares its pre-call
        // contents, so it must go stale the same way).
        let mut pos = [0u32; 3];
        let pos_ptr = (&mut pos as *mut u32) as u32;

        let table = rd32(lf_checker_rt::relocated(SECTOR_TABLE));
        let count = rd16(lf_checker_rt::relocated(SECTOR_COUNT)) as u32;
        if count == 0 {
            return 0;
        }
        let mut idx = 0u32;
        loop {
            let node = rd32(table.wrapping_add(idx.wrapping_mul(4)));
            let mut overlaps = true;
            if above(rdf(node), max_x) {
                overlaps = false;
            }
            if overlaps && above(rdf(node.wrapping_add(4)), max_y) {
                overlaps = false;
            }
            if overlaps && above(rdf(node.wrapping_add(8)), max_z) {
                overlaps = false;
            }
            if overlaps && above(min_x, rdf(node.wrapping_add(NODE_MAX))) {
                overlaps = false;
            }
            if overlaps && above(min_y, rdf(node.wrapping_add(NODE_MAX + 4))) {
                overlaps = false;
            }
            if overlaps && above(min_z, rdf(node.wrapping_add(NODE_MAX + 8))) {
                overlaps = false;
            }
            if overlaps {
                let mut ent = rd32(node.wrapping_add(NODE_HEAD));
                while ent != 0 {
                    let mut accept = true;
                    let veto = lf_checker_rt::callee_cdecl!(0, u32, ent);
                    if (veto as u8) != 0 {
                        accept = false;
                    }
                    if accept && rd8(ent) & FLAG_MASK == 0 {
                        accept = false;
                    }
                    if accept && rd32(ent.wrapping_add(ENT_MARKER)) != 0 {
                        accept = false;
                    }
                    if accept {
                        lf_checker_rt::callee_thiscall!(1, u32, ent, pos_ptr, 0u32);
                        let probe = lf_checker_rt::callee_thiscall!(2, u32, ent);
                        if (probe as u8) != 0 {
                            accept = false;
                        }
                    }
                    if accept {
                        let pz = f32::from_bits(pos[2]);
                        let dz = sub(cz, pz);
                        let adz = if above(0.0, dz) { abs_bits(dz) } else { dz };
                        let zlim = rdf(lf_checker_rt::relocated(Z_LIMIT));
                        if below_or_unordered(zlim, adz) {
                            accept = false;
                        } else {
                            let px = f32::from_bits(pos[0]);
                            let py = f32::from_bits(pos[1]);
                            let dx = sub(cx, px);
                            let dy = sub(cy, py);
                            let dz2 = mul(dz, dz);
                            let dx2 = mul(dx, dx);
                            let dy2 = mul(dy, dy);
                            let dist2 = add(add(dx2, dy2), dz2);
                            let rad2 = mul(rad, rad);
                            if below_or_unordered(rad2, dist2) {
                                accept = false;
                            } else {
                                let dist = dist2.sqrt();
                                if filter_cb != 0 {
                                    let f: extern "cdecl" fn(u32, u32, u32) -> u32 =
                                        unsafe { core::mem::transmute(filter_cb as usize) };
                                    let ok = f(ent, pos_ptr, user);
                                    if (ok as u8) == 0 {
                                        accept = false;
                                    }
                                }
                                if accept {
                                    if score_cb != 0 {
                                        let s: extern "cdecl" fn(u32, u32, u32) -> f32 =
                                            unsafe { core::mem::transmute(score_cb as usize) };
                                        let score = s(ent, pos_ptr, user);
                                        if above(best, score) {
                                            best = score;
                                            best_ent = ent;
                                        }
                                    } else if extra == 0 {
                                        if above(best, dist) {
                                            best = dist;
                                            best_ent = ent;
                                        }
                                    } else {
                                        let ok5 = lf_checker_rt::callee_cdecl!(5, u32, ent, extra, pos_ptr);
                                        if mode == 2 {
                                            if (ok5 as u8) != 0 && above(best, dist) {
                                                best = dist;
                                                best_ent = ent;
                                            }
                                        } else if mode == 1 {
                                            let d = if (ok5 as u8) != 0 {
                                                dist
                                            } else {
                                                add(
                                                    mul(dist, rdf(lf_checker_rt::relocated(MODE1_MUL))),
                                                    rdf(lf_checker_rt::relocated(MODE1_ADD)),
                                                )
                                            };
                                            if above(best, d) {
                                                best = d;
                                                best_ent = ent;
                                            }
                                        } else if above(best, dist) {
                                            best = dist;
                                            best_ent = ent;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ent = rd32(ent.wrapping_add(ENT_NEXT));
                }
            }
            idx = idx.wrapping_add(1);
            let count_now = rd16(lf_checker_rt::relocated(SECTOR_COUNT)) as u32;
            if !(idx < count_now) {
                break;
            }
        }
        best_ent
    }
});
