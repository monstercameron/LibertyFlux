// original: 0x00CC6BA0 euphoria_pick_best_match (proposed)

/// Pick the highest-scoring candidate whose tag is wanted, or null.
///
/// Resolves the wanted-tag array for `kind` through the table callee (which
/// also reports how many tags there are), then scans one candidate per tag:
/// each tag word is compared against the tag (`+0xc`) of every object the
/// enumerator callees yield in turn. A strictly greater score (the float at
/// `+0x58`, compared against the running best, which starts at a global
/// seed) keeps the object; anything else leaves the best unchanged. Returns
/// the winning object or null. A non-positive tag count returns null
/// immediately.
///
/// The original keeps the running best in its incoming argument slot on the
/// stack; the rewrite holds it in a local, so the stack comparison is off for
/// this function.
///
/// Original: 0x00CC6BA0 (stdcall, two stack words).
lf_checker_rt::export!(stdcall, rw_00cc6ba0(obj: u32, kind: u32) -> u32 {
    unsafe {
        const TABLE_CALLEE: u32 = 1;
        const FIRST_CALLEE: u32 = 2;
        const NEXT_CALLEE: u32 = 3;
        const ITER_AT: u32 = 0x78;
        const TAG_AT: u32 = 0x0c;
        const SCORE_AT: u32 = 0x58;
        const SEED_FLOAT: u32 = 0x00FE8D94;
        let mut tags = 0u32;
        let count = lf_checker_rt::callee_stdcall!(
            TABLE_CALLEE,
            u32,
            kind,
            &mut tags as *mut u32 as u32
        );
        let iter = (obj.wrapping_add(ITER_AT) as *const u32).read_unaligned();
        let mut best_bits =
            (lf_checker_rt::relocated(SEED_FLOAT) as *const u32).read_unaligned();
        let mut best = 0u32;
        if (count as i32) <= 0 {
            return best;
        }
        let mut i = 0u32;
        while (i as i32) < (count as i32) {
            let want = (tags.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned();
            let mut cand = lf_checker_rt::callee_thiscall!(FIRST_CALLEE, u32, iter);
            if cand != 0 {
                loop {
                    if (cand.wrapping_add(TAG_AT) as *const u32).read_unaligned() == want {
                        let score =
                            (cand.wrapping_add(SCORE_AT) as *const u32).read_unaligned();
                        if f32::from_bits(score) > f32::from_bits(best_bits) {
                            best = cand;
                            best_bits = score;
                        }
                    }
                    cand = lf_checker_rt::callee_thiscall!(NEXT_CALLEE, u32, iter);
                    if cand == 0 {
                        break;
                    }
                }
            }
            i = i.wrapping_add(1);
        }
        best
    }
});
