// original: 0x00d87630 ui_range_update (proposed)

/// Update a tracker's range index from its anchor distance, then dispatch on
/// the range threshold.
///
/// `obj` is the same tracker as in `rw_00d874f0` (`+0x20` anchor with the
/// reference point at `+0x30`/`+0x34`, own point at `+0xE4C`/`+0xE50`,
/// blend weight at `+0xE54`, threshold at `+0xE60`, signed/unsigned index
/// words at `+0xE68`/`+0xE6A`, owner at `+0xF50`). `a1` and `a2` are opaque
/// words forwarded to the downstream callees.
///
/// Algorithm: dist = length(own - anchor). Take the larger of the two index
/// words as floats (unordered comparison keeps the second on ties and NaN),
/// then clamped above by weight + dist (an ordered minimum with the
/// operands compared in the opposite order); truncate toward zero with
/// x86 convert semantics (out of range or NaN gives 0x80000000) and store
/// the low 16 bits into `+0xE68`. When the owner exists and its byte at
/// `+0x26A` has bit 0 set, replace the index with the unsigned word when it
/// is signed-greater. If the threshold is at least 0, call the six-argument
/// downstream callee with (obj, threshold, frame slot, 1, 0, a2) and return;
/// otherwise call the heading callee for the anchor delta and then the
/// five-argument callee with (obj, heading, dist, 1, a1). No value is
/// returned.
///
/// Original: 0x00D87630 (cdecl, three stack words).
lf_checker_rt::export!(cdecl, rw_00d87630(obj: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const OBJ_ANCHOR: u32 = 0x20;
        const OBJ_POS_X: u32 = 0x0e4c;
        const OBJ_POS_Y: u32 = 0x0e50;
        const OBJ_WEIGHT: u32 = 0x0e54;
        const OBJ_INDEX: u32 = 0x0e68;
        const OBJ_INDEX_U: u32 = 0x0e6a;
        const OBJ_THRESH: u32 = 0x0e60;
        const OBJ_OWNER: u32 = 0x0f50;
        const ANCH_REF_X: u32 = 0x30;
        const ANCH_REF_Y: u32 = 0x34;
        const OWNER_FLAG: u32 = 0x26a;
        const CAL_DOWN6: u32 = 1;
        const CAL_HEADING: u32 = 2;
        const CAL_DOWN5: u32 = 3;

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
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn bb(x: f32) -> f32 {
            core::hint::black_box(x)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            bb(a) - bb(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            bb(a) * bb(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            bb(a) + bb(b)
        }
        /// Exact x86 cvttss2si, including the indefinite value.
        #[inline(always)]
        fn cvtt(x: f32) -> i32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                i32::MIN
            } else {
                core::hint::black_box(x) as i32
            }
        }
        /// Original's ordered-max: keep the first only when ordered-above.
        #[inline(always)]
        fn pick_max(keep: f32, other: f32) -> f32 {
            if bb(keep) > bb(other) {
                keep
            } else {
                other
            }
        }
        /// Original's ordered-min: keep the first only when the second is
        /// ordered-above it (the compare order is swapped vs pick_max).
        #[inline(always)]
        fn pick_min(keep: f32, other: f32) -> f32 {
            if bb(other) > bb(keep) {
                keep
            } else {
                other
            }
        }

        let aux = rd32(obj + OBJ_ANCHOR);
        let pos_y = rdf(obj + OBJ_POS_Y);
        let pos_x = rdf(obj + OBJ_POS_X);
        let dy = sub(pos_y, rdf(aux + ANCH_REF_Y));
        let dx = sub(pos_x, rdf(aux + ANCH_REF_X));
        let dist = bb(add(mul(bb(dx), bb(dx)), mul(bb(dy), bb(dy)))).sqrt();
        let e68 = rd16(obj + OBJ_INDEX) as i16 as i32;
        let e6a = rd16(obj + OBJ_INDEX_U);
        let f68 = core::hint::black_box(e68) as f32;
        let f6a = core::hint::black_box(e6a as i16 as i32) as f32;
        let mut best = pick_max(f68, f6a);
        let cand = add(rdf(obj + OBJ_WEIGHT), dist);
        best = pick_min(best, cand);
        let trunc = cvtt(best);
        wr16(obj + OBJ_INDEX, trunc as u16);
        let owner = rd32(obj + OBJ_OWNER);
        if owner != 0 && rd8(owner + OWNER_FLAG) & 1 != 0 {
            let lo = trunc as u16;
            let v = if (e6a as i16) > (lo as i16) { e6a } else { lo };
            wr16(obj + OBJ_INDEX, v);
        }
        let thresh = rdf(obj + OBJ_THRESH);
        // comiss+jb: the jump is taken for less-than AND unordered, so the
        // fall-through (downstream) path needs an ordered >=, not !(a < b).
        if bb(thresh) >= bb(0.0) {
            let mut frame_slot = 0u32;
            lf_checker_rt::callee_cdecl!(
                CAL_DOWN6,
                u32,
                obj,
                thresh.to_bits(),
                &mut frame_slot as *mut u32 as u32,
                1u32,
                0u32,
                a2
            );
            return 0;
        }
        let dy2 = sub(pos_y, rdf(aux + ANCH_REF_Y));
        let dx2 = sub(pos_x, rdf(aux + ANCH_REF_X));
        let h: f32 = lf_checker_rt::callee_cdecl!(
            CAL_HEADING,
            f32,
            dx2.to_bits(),
            dy2.to_bits()
        );
        lf_checker_rt::callee_cdecl!(
            CAL_DOWN5,
            u32,
            obj,
            h.to_bits(),
            dist.to_bits(),
            1u32,
            a1
        );
        0
    }
});
