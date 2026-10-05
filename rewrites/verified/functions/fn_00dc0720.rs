// original: 0x00DC0720 ped_task_pick_best_scored (proposed)

/// Pick the best-scoring task candidate from a shared table.
///
/// `this` is the task owner object (flag word reached through the pointer at
/// `+0xC18`, bit 9 of the word at `+0x14` of that). `cand` points at a flags
/// word whose bit 1 selects a variant passed on to the fetch callee; its low
/// byte in the caller's argument slot is also patched with that bit by the
/// original (a stack-slot write a Rust rewrite cannot reproduce, so the
/// contract runs with the stack check off). `p1`/`p2` are filter values, `p3`
/// is forwarded to the confirm callee, the next two words are unread. The
/// last five words are: an optional 16-byte out record, an optional out float
/// for the winning score, a mandatory out word for the winning index, a
/// "confirm each improvement" flag, and a "strict" flag.
///
/// Behaviour: the fetch callee (cdecl, six arguments) returns how many of the
/// table's 16-byte records at the shared base are live. Each record holds
/// three floats; the score is `|x*w0 + y*w1 + z*w2 + c0|` scaled by 50 past a
/// threshold of 2, plus `(4 - (x*v0 + y*v1 + z*v2 + c1)) * 100` when the
/// second sum is below 4. The running best starts at ten million; an
/// improvement is kept directly when confirmation is off, otherwise the
/// confirm callee (thiscall) accepts or rejects it, and a rejection under the
/// strict flag sets a global byte and fails the whole call. On success the
/// winning index, record and score are written out and 1 is returned, else 0.
///
/// Original: 0x00DC0720 (thiscall, eleven stack words; words 4 and 5 unread).
lf_checker_rt::export!(thiscall, rw_00dc0720(this: u32, cand: u32, p1: u32, p2: u32, p3: u32, _u4: u32, _u5: u32, out_record: u32, out_score: u32, out_index: u32, need_check: u32, strict: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x17A1ED0;
        const W2_BASE: u32 = 0x17A2320;
        const W_BASE: u32 = 0x17A2330;
        const C_SECOND: u32 = 0x17A3384;
        const C_FIRST: u32 = 0x17A339C;
        const BEST_INIT: u32 = 0xE75900;
        const THRESH1: f32 = 2.0;
        const SCALE1: f32 = 50.0;
        const THRESH2: f32 = 4.0;
        const SCALE2: f32 = 100.0;
        const CONFIRM_TAG: u32 = 0x1057810;
        const STRICT_FLAG: u32 = 0x17A337E;
        const FETCH_CALLEE: u32 = 0;
        const CONFIRM_CALLEE: u32 = 1;
        const CHAIN_OFF: u32 = 0xC18;
        const CHAIN_FLAGS: u32 = 0x14;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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

        let pick_bit = ((rd32(cand) >> 1) & 1) as u32;
        // The original patches the low byte of its own incoming cand slot
        // with pick_bit and passes the patched word on; the slot write is
        // not reproducible from Rust (stack check off), the value is.
        let cand_patched = (cand & 0xFFFF_FF00) | pick_bit;
        let table = lf_checker_rt::relocated(TABLE);
        let count = lf_checker_rt::callee_cdecl!(FETCH_CALLEE, u32, cand, p1, p2, 0, cand_patched, table) as i32;
        let mut best = rdf(lf_checker_rt::relocated(BEST_INIT));
        if count <= 0 {
            return 0;
        }
        let w0 = rdf(lf_checker_rt::relocated(W_BASE));
        let w1 = rdf(lf_checker_rt::relocated(W_BASE + 4));
        let w2 = rdf(lf_checker_rt::relocated(W_BASE + 8));
        let v0 = rdf(lf_checker_rt::relocated(W2_BASE));
        let v1 = rdf(lf_checker_rt::relocated(W2_BASE + 4));
        let v2 = rdf(lf_checker_rt::relocated(W2_BASE + 8));
        let c0 = rdf(lf_checker_rt::relocated(C_FIRST));
        let c1 = rdf(lf_checker_rt::relocated(C_SECOND));
        let mut best_idx: i32 = -1;
        let mut i: i32 = 0;
        while i < count {
            let rec = table.wrapping_add((i as u32).wrapping_mul(16));
            let x = rdf(rec);
            let y = rdf(rec + 4);
            let z = rdf(rec + 8);
            let mut s = add(add(add(mul(x, w0), mul(y, w1)), mul(z, w2)), c0);
            s = f32::from_bits(s.to_bits() & 0x7FFF_FFFF);
            let mut score = 0.0f32;
            if s > THRESH1 {
                score = mul(s, SCALE1);
            }
            let mut t = add(add(add(mul(x, v0), mul(y, v1)), mul(z, v2)), c1);
            if THRESH2 > t {
                t = sub(t, THRESH2);
                t = f32::from_bits(t.to_bits() ^ 0x8000_0000);
                t = mul(t, SCALE2);
                score = add(score, t);
            }
            if best > score {
                if (need_check & 0xFF) == 0 {
                    best = score;
                    best_idx = i;
                } else {
                    let chain = rd32(this.wrapping_add(CHAIN_OFF));
                    let flag = rd32(chain.wrapping_add(CHAIN_FLAGS));
                    let bit = ((flag >> 9) & 1) as u32;
                    let tag = lf_checker_rt::relocated(CONFIRM_TAG);
                    let ok: u32 = lf_checker_rt::callee_thiscall!(CONFIRM_CALLEE, u32, this, p3, rec, tag, bit);
                    if (ok & 0xFF) != 0 {
                        best = score;
                        best_idx = i;
                    } else if (strict & 0xFF) != 0 {
                        ((lf_checker_rt::relocated(STRICT_FLAG)) as *mut u8).write(1);
                        return 0;
                    }
                }
            }
            i += 1;
        }
        if best_idx == -1 {
            return 0;
        }
        wr32(out_index, best_idx as u32);
        if out_record != 0 {
            let src = table.wrapping_add((best_idx as u32).wrapping_mul(16));
            wr32(out_record, rd32(src));
            wr32(out_record + 4, rd32(src + 4));
            wr32(out_record + 8, rd32(src + 8));
            wr32(out_record + 12, rd32(src + 12));
        }
        if out_score != 0 {
            wrf(out_score, best);
        }
        1
    }
});
