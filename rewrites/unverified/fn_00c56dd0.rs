// original: 0x00c56dd0 peds_task_pick_nearest_row (proposed)

/// Find the nearest acceptable row of a task table to a point.
///
/// The table header lives behind the global at `TABLE_HDR` (row base at
/// `+0`, flag bytes at `+4`, row count at `+8`, row stride at `+0xC`) and is
/// scanned from the last row down to the first. A row is skipped when its
/// flag byte has bit `0x80` set, when its address computes to zero, when
/// the first non-empty kind slot of its object (five words at `+0x44` past
/// `+0x224`) answers `SKIP_KIND` through virtual slot `+0xC`, when its
/// squared distance from the point is not strictly below the running best
/// (starting at the largest finite float, unordered comparisons keeping the
/// old best), or when either of two scripted predicate calls rejects it.
/// The survivor with the smallest distance is returned, or zero.
///
/// The distance is `((dy * dy) + (dx * dx)) + (dz * dz)` with the
/// differences taken from the point's components minus the row object's
/// three floats at `+0x30`, in the original's operand order, pinned through
/// `core::hint::black_box` so NaN payloads propagate identically. The
/// second predicate takes the point, a scratch buffer the rewrite builds
/// on its own frame, a zero word and the row. The buffer holds three
/// four-word rows (an identity in the first three columns; the original
/// never writes the fourth column, leaving words 3, 7 and 11
/// uninitialized, so the contract does not compare them and the rewrite
/// holds zero there) followed by three zero words.
///
/// Original: 0x00C56DD0 (cdecl, one stack word = point with three floats,
/// returns the winning row address or zero).
lf_checker_rt::export!(cdecl, rw_00c56dd0(pt: u32) -> u32 {
    unsafe {
        const TABLE_HDR: u32 = 0x018B6F1C;
        const BEST_INIT: u32 = 0x00FE8D18;
        const SKIP_FLAG: u8 = 0x80;
        const SKIP_KIND: u32 = 0xE9;
        const VT_SLOT: u32 = 0x0C;
        const KIND_WORDS: u32 = 5;
        const KIND0: u32 = 0x44;
        const OBJ_OFF: u32 = 0x224;
        const SUB_OFF: u32 = 0x20;
        const CAL_PRE: u32 = 1;
        const CAL_TEST: u32 = 2;
        const CAL_MATRIX: u32 = 3;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
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
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        let hdr = rd32(lf_checker_rt::relocated(TABLE_HDR));
        let rows = rd32(hdr);
        let flags = rd32(hdr.wrapping_add(4));
        let mut left = rd32(hdr.wrapping_add(8));
        let stride = rd32(hdr.wrapping_add(0xC));
        let mut best = f32::from_bits(rd32(lf_checker_rt::relocated(BEST_INIT)));
        let mut winner: u32 = 0;
        if left == 0 {
            return 0;
        }
        loop {
            left = left.wrapping_sub(1);
            let row = (left.wrapping_mul(stride)).wrapping_add(rows);
            if rd8(flags.wrapping_add(left)) & SKIP_FLAG == 0 && row != 0 {
                let obj = rd32(row.wrapping_add(OBJ_OFF));
                let mut kind_ok = true;
                let mut j = 0u32;
                while j < KIND_WORDS {
                    let t = rd32(obj.wrapping_add(KIND0).wrapping_add(j * 4));
                    if t != 0 {
                        let kind: extern "thiscall" fn(u32) -> u32 =
                            core::mem::transmute(rd32(rd32(t) + VT_SLOT) as usize);
                        if kind(t) == SKIP_KIND {
                            kind_ok = false;
                        }
                        break;
                    }
                    j += 1;
                }
                if kind_ok {
                    let sobj = rd32(row.wrapping_add(SUB_OFF));
                    let dy = sub(rdf(pt.wrapping_add(4)), rdf(sobj.wrapping_add(0x34)));
                    let dx = sub(rdf(pt), rdf(sobj.wrapping_add(0x30)));
                    let dz = sub(rdf(pt.wrapping_add(8)), rdf(sobj.wrapping_add(0x38)));
                    let d = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
                    if d < best {
                        let r: u32 = lf_checker_rt::callee_cdecl!(CAL_PRE, u32, pt);
                        let ok: u32 = lf_checker_rt::callee_cdecl!(CAL_TEST, u32, r, row);
                        if (ok as u8) != 0 {
                            let mut m: [u32; 15] = [0; 15];
                            m[0] = 0x3F800000;
                            m[5] = 0x3F800000;
                            m[10] = 0x3F800000;
                            let done: u32 = lf_checker_rt::callee_cdecl!(
                                CAL_MATRIX, u32, pt, m.as_mut_ptr() as u32, 0u32, row
                            );
                            if (done as u8) != 0 {
                                best = d;
                                winner = row;
                            }
                        }
                    }
                }
            }
            if left == 0 {
                break;
            }
        }
        winner
    }
});
