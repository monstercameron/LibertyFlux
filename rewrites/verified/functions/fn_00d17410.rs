// original: 0x00D17410 cover_slot_path_filter (proposed)

/// Scan five cover slots for a sharp direction change, filter the slot
/// flags by what was found, and range-check the slot mid values.
///
/// `obj` holds five 0x50-byte slots (flag byte at +0, a 2D point at +0x30,
/// a mid value at +0x40). When the low byte of `flag` is zero, the points
/// of set slots are walked in order: the first sets the anchor, the second
/// sets the unit direction (scaled by 1/length, or zero for a repeated
/// point), and each further point forms a dot product with the running
/// direction. A dot below 0.707 records the slot index (only the first
/// time; a second one aborts the scan), a dot above 0.95 bumps one of two
/// counters. Afterwards the flags above (or, when the high-dot counter
/// wins, below) the recorded index are cleared and the function returns 1;
/// otherwise it returns 0. The tail always runs: over slots 1-3 it takes
/// the max (from 0.0) and min (from 9999.0) of the mid values of set slots,
/// and when all three are set and the game's range limit exceeds the spread,
/// slots 0 and 4 are cleared when their mid value sticks out past the max
/// by more than the limit (each also clearing a shadow flag at +0x190 and
/// +0x2D0). Leaf, no calls.
///
/// Original: 0x00D17410 (cdecl, two stack words, returns a byte in al).
lf_checker_rt::export!(cdecl, rw_00D17410(obj: u32, flag: u32) -> u32 {
    unsafe {
        const C_NORM: u32 = 0xfe88e8;
        const C_DOT_LO: u32 = 0xfe8878;
        const C_DOT_HI: u32 = 0xfe88c4;
        const C_MIN_INIT: u32 = 0xec8d70;
        const C_RANGE: u32 = 0x1053e78;
        const SLOT_STRIDE: u32 = 0x50;
        const SLOT_COUNT: u32 = 5;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
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
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn tail(obj: u32, retflag: u8) -> u32 {
            unsafe {
                let mut maxv = 0.0f32;
                let mut minv = f32::from_bits(rd32(lf_checker_rt::relocated(C_MIN_INIT)));
                let mut n: u32 = 0;
                let mut s = 1u32;
                while s <= 3 {
                    if rd8(obj + s * SLOT_STRIDE) != 0 {
                        let v = rdf(obj + s * SLOT_STRIDE + 0x40);
                        n += 1;
                        if !(maxv > v) {
                            maxv = v;
                        }
                        if !(v > minv) {
                            minv = v;
                        }
                    }
                    s += 1;
                }
                if n != 3 {
                    return retflag as u32;
                }
                let range = sub(maxv, minv);
                let limit = f32::from_bits(rd32(lf_checker_rt::relocated(C_RANGE)));
                if !(limit > range) {
                    return retflag as u32;
                }
                if rd8(obj) != 0 {
                    let d = sub(rdf(obj + 0x40), maxv);
                    if d > limit {
                        wr8(obj, 0);
                        wr8(obj + 0x190, 0);
                    }
                }
                if rd8(obj + 0x140) == 0 {
                    return retflag as u32;
                }
                let d2 = sub(rdf(obj + 0x180), maxv);
                if d2 > limit {
                    wr8(obj + 0x140, 0);
                    wr8(obj + 0x2d0, 0);
                }
                retflag as u32
            }
        }

        let mut retflag: u8 = 0;
        if (flag & 0xff) != 0 {
            return tail(obj, retflag);
        }
        let norm = f32::from_bits(rd32(lf_checker_rt::relocated(C_NORM)));
        let dot_lo = f32::from_bits(rd32(lf_checker_rt::relocated(C_DOT_LO)));
        let dot_hi = f32::from_bits(rd32(lf_checker_rt::relocated(C_DOT_HI)));
        let mut got_first = false;
        let mut got_second = false;
        let mut last_px = 0.0f32;
        let mut last_py = 0.0f32;
        let mut count_a: u32 = 0;
        let mut edx: i32 = -1;
        let mut edi: u32 = 0;
        let mut dirx = 0.0f32;
        let mut diry = 0.0f32;
        let mut z2 = 0.0f32;
        let mut counter: u32 = 0;
        while counter < SLOT_COUNT {
            let slot = obj + counter * SLOT_STRIDE;
            if rd8(slot) != 0 {
                if !got_first {
                    last_px = rdf(slot + 0x30);
                    last_py = rdf(slot + 0x34);
                    got_first = true;
                } else if !got_second {
                    let qx = rdf(slot + 0x30);
                    let qy = rdf(slot + 0x34);
                    let dx = sub(qx, last_px);
                    let dy = sub(qy, last_py);
                    let len_sq = add(mul(dy, dy), mul(dx, dx));
                    let n = if len_sq != 0.0 { div(norm, len_sq.sqrt()) } else { 0.0 };
                    dirx = mul(dx, n);
                    diry = mul(dy, n);
                    z2 = mul(n, 0.0);
                    last_px = qx;
                    last_py = qy;
                    got_second = true;
                    edi = 1;
                } else {
                    let qx = rdf(slot + 0x30);
                    let qy = rdf(slot + 0x34);
                    let dx = sub(qx, last_px);
                    let dy = sub(qy, last_py);
                    let len_sq = add(mul(dy, dy), mul(dx, dx));
                    let n = if len_sq != 0.0 { div(norm, len_sq.sqrt()) } else { 0.0 };
                    let dxu = mul(dx, n);
                    let dyu = mul(dy, n);
                    let t0 = mul(dyu, diry);
                    let z3 = mul(n, 0.0);
                    let t1 = mul(dxu, dirx);
                    let mut dot = add(t0, t1);
                    let t2 = mul(z3, z2);
                    dot = add(dot, t2);
                    if !(dot_lo > dot) {
                        if dot > dot_hi {
                            if edx == -1 {
                                edi += 1;
                            } else {
                                count_a += 1;
                            }
                        }
                    } else if edx != -1 {
                        return tail(obj, retflag);
                    } else {
                        count_a += 1;
                        edx = (counter as i32) - 1;
                        z2 = z3;
                        dirx = dxu;
                        diry = dyu;
                    }
                }
            }
            counter += 1;
        }
        if edx == -1 {
            return tail(obj, retflag);
        }
        if edi > count_a || edi >= 2 {
            retflag = 1;
            if edx < 0 {
                wr8(obj, 0);
            }
            if edx < 1 {
                wr8(obj + 0x50, 0);
            }
            if edx < 2 {
                wr8(obj + 0xa0, 0);
            }
            if edx < 3 {
                wr8(obj + 0xf0, 0);
            }
            if edx < 4 {
                wr8(obj + 0x140, 0);
            }
            return tail(obj, retflag);
        }
        if count_a <= edi {
            return tail(obj, retflag);
        }
        retflag = 1;
        if edx > 0 {
            wr8(obj, 0);
        }
        if edx > 1 {
            wr8(obj + 0x50, 0);
        }
        if edx > 2 {
            wr8(obj + 0xa0, 0);
        }
        if edx > 3 {
            wr8(obj + 0xf0, 0);
        }
        if edx > 4 {
            wr8(obj + 0x140, 0);
        }
        tail(obj, retflag)
    }
});
