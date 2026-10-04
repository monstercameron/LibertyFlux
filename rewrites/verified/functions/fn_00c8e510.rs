// original: 0x00C8E510 ped_task_collect_scored (proposed)

/// Scan the ped-task spatial index, score every nearby entity and collect them.
///
/// `vec` points to three floats (x, y, z); `radius` is the search radius and
/// `mindist` a minimum distance. The index is the same sector array as in
/// `ped_task_best_in_radius` (`SECTOR_TABLE` / `SECTOR_COUNT`), with the same
/// box-overlap test and entity chains through `+0x1c`.
///
/// Each entity of an overlapping node is filtered in order: the exclusion
/// list (`excl_list`, `excl_count` entries, skipped when the count is not
/// positive), callee 0 (cdecl, vetoes on a non-zero low byte), callee 1
/// (thiscall, same), the flag bit at `+0x00` shifted right by 10 (vetoes
/// when set), and, unless `flag_b` is set, the low three flag bits combined
/// with the marker dword at `+0x14` (vetoes when the bits are set and the
/// marker is non-zero). Callee 2 (thiscall) then writes the entity position
/// into the scratch slot and one is added to its height.
///
/// Scoring: the distance must lie strictly between `mindist` and `radius`
/// (ordered comparisons). Callee 4 (thiscall) observes the difference
/// vector, callee 5 (thiscall) turns it into a float deviation on the x87
/// stack, and the absolute deviation must be strictly below `maxdev`. The
/// score is `(1 - (dist - mindist) / (radius - mindist)) + (1 - dev /
/// maxdev)`, plus one when the flag bits are set and the marker is non-zero.
/// Callee 6 (cdecl) then probes the candidate: mode 2 keeps it only on a
/// non-zero answer, mode 1 halves the score on a zero answer, any other
/// mode keeps the plain score.
///
/// Accepted candidates are appended to the result tables: the entity
/// pointer at `RESULT_PTRS[accepted]` and the running score total at
/// `RESULT_TOTALS[accepted + 1]` (at most 48 entries). Unless `flag_c` is
/// set, the best score so far (starting from `BEST_INIT`) and its index are
/// tracked. Returns 0 when nothing was accepted or the index is empty. When
/// `flag_c` is clear the function returns the best entry's pointer (or the
/// word just below the table when no score ever beat the seed). When
/// `flag_c` is set, callee 7 (cdecl) supplies an integer that is scaled by
/// `SCAN_MUL` and the score total into a threshold, and the function
/// returns the first entry whose total reaches it, refreshing `out` through
/// callee 8 (thiscall) when `out` is non-null.
///
/// All threshold tests skip only on the ordered outcome, so NaN takes the
/// rejection branch in every case. Float operation order matches the
/// original exactly.
///
/// Original: 0x00C8E510 (cdecl, fourteen stack words).
lf_checker_rt::export!(cdecl, rw_00C8E510(
    out: u32,
    vec: u32,
    opaque: u32,
    maxdev: u32,
    mindist: u32,
    radius: u32,
    flag_b: u32,
    flag_c: u32,
    extra: u32,
    mode: u32,
    excl_list: u32,
    excl_count: u32,
    filter_cb: u32,
    user: u32,
) -> u32 {
    unsafe {
        const SECTOR_TABLE: u32 = 0x1713A9C;
        const SECTOR_COUNT: u32 = 0x1713AA0;
        const BEST_INIT: u32 = 0xFE8638;
        const ONE: u32 = 0xFE88E8;
        const MODE1_MUL: u32 = 0xFE8830;
        const SCAN_MUL: u32 = 0xFE8684;
        const SIGN_MASK: u32 = 0x8000_0000;
        const NODE_MAX: u32 = 0x10;
        const NODE_HEAD: u32 = 0x20;
        const ENT_MARKER: u32 = 0x14;
        const ENT_NEXT: u32 = 0x1c;
        const RESULT_PTRS: u32 = 0x171B9C0;
        const RESULT_TOTALS: u32 = 0x171BA7C;
        const MAX_RESULTS: u32 = 0x30;

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
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
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
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        /// `comiss p, q` + `ja`: ordered-above only.
        #[inline(always)]
        fn above(p: f32, q: f32) -> bool {
            p > q
        }
        /// `comiss p, q` + `jbe`: below, equal or unordered.
        #[inline(always)]
        fn below_or_unordered(p: f32, q: f32) -> bool {
            !(p > q)
        }
        /// `comiss p, q` + `jae`: ordered above-or-equal.
        #[inline(always)]
        fn above_or_equal(p: f32, q: f32) -> bool {
            p >= q
        }

        let vx = rdf(vec);
        let vy = rdf(vec.wrapping_add(4));
        let vz = rdf(vec.wrapping_add(8));
        let rad = f32::from_bits(radius);
        let min_x = sub(vx, rad);
        let min_y = sub(vy, rad);
        let min_z = sub(vz, rad);
        let max_x = add(vx, rad);
        let max_y = add(vy, rad);
        let max_z = add(vz, rad);
        let mind = f32::from_bits(mindist);
        let maxd = f32::from_bits(maxdev);

        let table = rd32(lf_checker_rt::relocated(SECTOR_TABLE));
        let count = rd16(lf_checker_rt::relocated(SECTOR_COUNT)) as u32;
        if count == 0 {
            return 0;
        }
        let mut best = rdf(lf_checker_rt::relocated(BEST_INIT));
        let mut best_idx = 0xffff_ffffu32;
        let mut total = 0.0f32;
        let mut accepted = 0u32;
        // Scratch position slot, reused across candidates like the
        // original's frame slot (a callee snapshot compares pre-call contents).
        let mut pos = [0u32; 3];
        let pos_ptr = (&mut pos as *mut u32) as u32;
        let mut diff = [0u32; 3];
        let diff_ptr = (&mut diff as *mut u32) as u32;
        let one = rdf(lf_checker_rt::relocated(ONE));

        let mut idx = 0u32;
        loop {
            if accepted >= MAX_RESULTS {
                break;
            }
            {
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
                        let mut cont = true;
                        // Exclusion list (signed count).
                        if (excl_count as i32) > 0 {
                            let mut i = 0u32;
                            while i < excl_count {
                                if rd32(excl_list.wrapping_add(i.wrapping_mul(4))) == ent {
                                    cont = false;
                                    break;
                                }
                                i = i.wrapping_add(1);
                            }
                        }
                        if cont {
                            let veto = lf_checker_rt::callee_cdecl!(0, u32, ent);
                            if (veto as u8) != 0 {
                                cont = false;
                            }
                        }
                        if cont {
                            let probe = lf_checker_rt::callee_thiscall!(1, u32, ent);
                            if (probe as u8) != 0 {
                                cont = false;
                            }
                        }
                        if cont {
                            let flags = rd32(ent);
                            if ((flags >> 10) & 1) != 0 {
                                cont = false;
                            } else if (flag_b as u8) == 0 {
                                if (flags as u8) & 7 != 0 && rd32(ent.wrapping_add(ENT_MARKER)) != 0
                                {
                                    cont = false;
                                }
                            }
                        }
                        if cont {
                            lf_checker_rt::callee_thiscall!(2, u32, ent, pos_ptr, 0u32);
                            let pz = add(f32::from_bits(pos[2]), one);
                            pos[2] = pz.to_bits();
                            if filter_cb != 0 {
                                let f: extern "cdecl" fn(u32, u32, u32) -> u32 =
                                    unsafe { core::mem::transmute(filter_cb as usize) };
                                let ok = f(ent, pos_ptr, user);
                                if (ok as u8) == 0 {
                                    cont = false;
                                }
                            }
                        }
                        if cont {
                            let px = f32::from_bits(pos[0]);
                            let py = f32::from_bits(pos[1]);
                            let pz = f32::from_bits(pos[2]);
                            let dy = sub(py, vy);
                            let dx = sub(px, vx);
                            let dz = sub(pz, vz);
                            diff[0] = dx.to_bits();
                            diff[1] = dy.to_bits();
                            diff[2] = dz.to_bits();
                            let dy2 = mul(dy, dy);
                            let dx2 = mul(dx, dx);
                            let dz2 = mul(dz, dz);
                            let dist2 = add(add(dy2, dx2), dz2);
                            let dist = dist2.sqrt();
                            if below_or_unordered(rad, dist) {
                                cont = false;
                            } else if below_or_unordered(dist, mind) {
                                cont = false;
                            } else {
                                lf_checker_rt::callee_thiscall!(4, u32, diff_ptr);
                                let dv = lf_checker_rt::callee_thiscall!(5, f32, diff_ptr, opaque);
                                let adev = if above(0.0, dv) {
                                    f32::from_bits(dv.to_bits() ^ SIGN_MASK)
                                } else {
                                    dv
                                };
                                if below_or_unordered(maxd, adev) {
                                    cont = false;
                                } else {
                                    let t1 = div(sub(dist, mind), sub(rad, mind));
                                    let t2 = div(dv, maxd);
                                    let mut score = add(sub(one, t1), sub(one, t2));
                                    let flags = rd32(ent);
                                    if (flags as u8) & 7 != 0
                                        && rd32(ent.wrapping_add(ENT_MARKER)) != 0
                                    {
                                        score = add(score, one);
                                    }
                                    let ok6 =
                                        lf_checker_rt::callee_cdecl!(6, u32, ent, extra, pos_ptr);
                                    if mode == 2 {
                                        if (ok6 as u8) == 0 {
                                            cont = false;
                                        }
                                    } else if mode == 1 {
                                        if (ok6 as u8) == 0 {
                                            score = mul(
                                                score,
                                                rdf(lf_checker_rt::relocated(MODE1_MUL)),
                                            );
                                        }
                                    }
                                    if cont {
                                        if (flag_c as u8) == 0 && above(score, best) {
                                            best = score;
                                            best_idx = accepted;
                                        }
                                        total = add(score, total);
                                        let ptrs = lf_checker_rt::relocated(RESULT_PTRS);
                                        let tots = lf_checker_rt::relocated(RESULT_TOTALS);
                                        wr32(ptrs.wrapping_add(accepted.wrapping_mul(4)), ent);
                                        accepted = accepted.wrapping_add(1);
                                        wrf(tots.wrapping_add(accepted.wrapping_mul(4)), total);
                                        if accepted >= MAX_RESULTS {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        if accepted >= MAX_RESULTS {
                            break;
                        }
                        ent = rd32(ent.wrapping_add(ENT_NEXT));
                    }
                }
            }
            idx = idx.wrapping_add(1);
            let count_now = rd16(lf_checker_rt::relocated(SECTOR_COUNT)) as u32;
            if !(idx < count_now) {
                break;
            }
        }
        if accepted == 0 {
            return 0;
        }
        if (flag_c as u8) == 0 {
            let ptrs = lf_checker_rt::relocated(RESULT_PTRS);
            return rd32(ptrs.wrapping_add(best_idx.wrapping_mul(4)));
        }
        let n = lf_checker_rt::callee_cdecl!(7, u32,);
        let mut thresh = (n as i32) as f32;
        thresh = mul(thresh, rdf(lf_checker_rt::relocated(SCAN_MUL)));
        thresh = mul(thresh, total);
        let tots = lf_checker_rt::relocated(RESULT_TOTALS);
        let mut pick = accepted;
        let mut i = 0u32;
        while i < accepted {
            // Totals are stored one slot up from the pointers.
            let t = rdf(tots.wrapping_add(4).wrapping_add(i.wrapping_mul(4)));
            if above_or_equal(t, thresh) {
                pick = i;
                break;
            }
            i = i.wrapping_add(1);
        }
        if pick >= accepted {
            return 0;
        }
        if out != 0 {
            // The original passes whatever ecx holds here (the mode word on
            // every path that reaches this call); the stub ignores it.
            lf_checker_rt::callee_thiscall!(8, u32, mode, out, 0u32);
        }
        let ptrs = lf_checker_rt::relocated(RESULT_PTRS);
        rd32(ptrs.wrapping_add(pick.wrapping_mul(4)))
    }
});
