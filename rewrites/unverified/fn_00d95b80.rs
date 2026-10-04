// original: 0x00d95b80 collect_matching_rows (proposed)

/// Collect rows matching a query into the accumulator object.
///
/// `this` carries the wanted 16-bit row id at `+0xc42`. `a0` is passed
/// through as the callee object for the two predicate calls. `a1` is the
/// accumulator: three floats at `+0x30` (length-squared against 0.25), a
/// weight at `+0x40` (against 0), and result arrays filled at `+0x4c`,
/// `+0x8c` and `+(count+0xd)*0x10`, with the count at `+0x44` (at most
/// 0x10 entries). `a2` holds the query array base at `+0x64`. `a3` holds
/// flag bits at `+0` (bit mask 0x1e00000 doubles as the iteration count in
/// bits 21-24) and the row-index base at `+4`.
///
/// Each iteration queries the row helper (callee 1) for one index word and
/// exits the iteration unless the low 12 bits, the high word and the flag
/// bits select a row; resolves the row's object (callee 2, nullable);
/// adopts the element `word*40` bytes into that object's table when its id
/// word differs from the wanted id; then classifies by (length-squared
/// above 0.25, weight above 0): long-and-set stores the index and the
/// helper float (callee 3), short-and-set checks the predicate (callee 5)
/// and either recurses (callee 7, the self call, intercepted) or stores,
/// long-and-set fetches a vector (callee 4), normalises it (callee 6) and
/// stores by its dot product against 0.86. A full accumulator or a refused
/// recursion returns 0; an empty flag word or an exhausted loop returns 1.
///
/// Convention: thiscall with four stack words; only the low result byte is
/// significant. Float operation order is the original's; NaN-aware
/// comparisons replicate the original's jump conditions bit for bit.
lf_checker_rt::export!(thiscall, rw_00d95b80(this: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const FLAG_MASK: u32 = 0x01e0_0000;
        const INDEX_MASK: u32 = 0x0001_ffff;
        const QUERY_BASE_OFF: u32 = 0x64;
        const WANT_ID_OFF: u32 = 0xc42;
        const TABLE_OFF: u32 = 0x6c;
        const ELEM_STRIDE: u32 = 40;
        const ELEM_ID_OFF: u32 = 8;
        const ELEM_PROBE_OFF: u32 = 0x10;
        const ELEM_CLASS_OFF: u32 = 0x1c;
        const ACC_POS: u32 = 0x20;
        const ACC_DIR: u32 = 0x30;
        const ACC_W: u32 = 0x40;
        const ACC_COUNT: u32 = 0x44;
        const ACC_CLASS: u32 = 0x4c;
        const ACC_SCORE: u32 = 0x8c;
        const ACC_VEC_BASE: u32 = 0x0d;
        const ACC_VEC_STRIDE: u32 = 16;
        const MAX_ROWS: i32 = 0x10;
        const QUERY_WORD: u32 = 0xffff_0fff;
        const LEN_SQ_LIMIT_VA: u32 = 0x00fe_87e4;
        const SCORE_LIMIT_VA: u32 = 0x016b_6b18;
        const DOT_LIMIT_VA: u32 = 0x0104_8268;
        const DIV_MAGIC: i64 = 0x6666_6667;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u32) {
            unsafe { (a as *mut u16).write_unaligned(v as u16) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
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
        /// `jb` after `comiss a, b`: taken when below or unordered.
        #[inline(always)]
        fn below(a: f32, b: f32) -> bool {
            !(core::hint::black_box(a) >= core::hint::black_box(b))
        }
        /// `jbe` after `comiss a, b`: taken when below, equal or unordered.
        #[inline(always)]
        fn below_eq(a: f32, b: f32) -> bool {
            !(core::hint::black_box(a) > core::hint::black_box(b))
        }
        /// The original's signed quotient idiom (magic multiply, shift,
        /// sign adjust), bit for bit.
        #[inline(always)]
        fn magic_quot(diff: u32) -> u32 {
            let prod = DIV_MAGIC.wrapping_mul(diff as i32 as i64);
            let q = (prod >> 32) as i32 >> 4;
            ((q as u32) >> 31).wrapping_add(q as u32)
        }

        let esi = a1;
        if rd32(a3) & FLAG_MASK == 0 {
            return 1;
        }
        // Frame buffers that persist across iterations, as in the original.
        let mut buf = [0u32; 2];
        let mut fbuf = [0u32; 4];
        let mut counter = 0u32;
        loop {
            buf[0] = QUERY_WORD;
            buf[1] = (buf[1] | 0x0fff_ffff) & 0xefff_ffff;
            let base = rd32(a2 + QUERY_BASE_OFF);
            let idx = (rd32(a3 + 4) & INDEX_MASK).wrapping_add(counter);
            let query = base.wrapping_add(idx.wrapping_mul(8));
            lf_checker_rt::callee_thiscall!(1, u32, query, buf.as_mut_ptr() as u32);
            let edi = buf[0];
            let lo12 = edi & 0xfff;
            let word = (edi >> 16) & 0xffff;
            let mut deep = lo12 != 0xfff && word != 0xffff && edi & 0x6000 == 0;
            if deep {
                let obj: u32 = lf_checker_rt::callee_cdecl!(2, u32, lo12);
                if obj == 0 {
                    deep = false;
                } else {
                    let fx = rdf(esi + ACC_DIR);
                    let fy = rdf(esi + ACC_DIR + 4);
                    let fz = rdf(esi + ACC_DIR + 8);
                    let lensq = add(add(mul(fx, fx), mul(fy, fy)), mul(fz, fz));
                    let fw = rdf(esi + ACC_W);
                    let long =
                        lensq > f32::from_bits(lf_checker_rt::global::<u32>(LEN_SQ_LIMIT_VA).read());
                    let set = fw > 0.0;
                    let table = rd32(obj + TABLE_OFF);
                    let elem = table.wrapping_add(word.wrapping_mul(5).wrapping_mul(8));
                    let want = rd16(this + WANT_ID_OFF);
                    if rd16(elem + ELEM_ID_OFF) == want {
                        deep = false;
                    } else {
                        wr16(elem + ELEM_ID_OFF, want);
                        let b1 = long && !set;
                        let b2 = !long && set;
                        let b3 = long && set;
                        if b1 {
                            let count = rd32(esi + ACC_COUNT);
                            wr32(
                                esi + count.wrapping_mul(4) + ACC_CLASS,
                                rd16(elem + ELEM_CLASS_OFF) & 0xf,
                            );
                            let fv: f32 = lf_checker_rt::callee_stdcall!(
                                3,
                                f32,
                                obj,
                                magic_quot(elem.wrapping_sub(table))
                            );
                            wrf(esi + count.wrapping_mul(4) + ACC_SCORE, fv);
                            let dst = esi
                                + rd32(esi + ACC_COUNT)
                                    .wrapping_add(ACC_VEC_BASE)
                                    .wrapping_mul(ACC_VEC_STRIDE);
                            lf_checker_rt::callee_thiscall!(4, u32, obj, elem, dst);
                            let grown = rd32(esi + ACC_COUNT).wrapping_add(1);
                            wr32(esi + ACC_COUNT, grown);
                            if grown as i32 >= MAX_ROWS {
                                return 0;
                            }
                        } else if b2 {
                            let ok: u32 = lf_checker_rt::callee_thiscall!(
                                5,
                                u32,
                                a0,
                                elem.wrapping_add(ELEM_PROBE_OFF)
                            );
                            if ok & 0xff != 0 {
                                let fv: f32 = lf_checker_rt::callee_stdcall!(
                                    3,
                                    f32,
                                    obj,
                                    magic_quot(elem.wrapping_sub(table))
                                );
                                if below(fv, f32::from_bits(lf_checker_rt::global::<u32>(SCORE_LIMIT_VA).read())) {
                                    let r: u32 = lf_checker_rt::callee_thiscall!(
                                        7, u32, this, a0, esi, obj, elem
                                    );
                                    if r & 0xff == 0 {
                                        return 0;
                                    }
                                } else {
                                    let count = rd32(esi + ACC_COUNT);
                                    wr32(
                                        esi + count.wrapping_mul(4) + ACC_CLASS,
                                        rd16(elem + ELEM_CLASS_OFF) & 0xf,
                                    );
                                    wrf(esi + count.wrapping_mul(4) + ACC_SCORE, fv);
                                    let dst = esi
                                        + count
                                            .wrapping_add(ACC_VEC_BASE)
                                            .wrapping_mul(ACC_VEC_STRIDE);
                                    lf_checker_rt::callee_thiscall!(4, u32, elem, elem, dst);
                                    let grown = rd32(esi + ACC_COUNT).wrapping_add(1);
                                    wr32(esi + ACC_COUNT, grown);
                                    if grown as i32 >= MAX_ROWS {
                                        return 0;
                                    }
                                    let r: u32 = lf_checker_rt::callee_thiscall!(
                                        7, u32, this, a0, esi, obj, elem
                                    );
                                    if r & 0xff == 0 {
                                        return 0;
                                    }
                                }
                            }
                        } else if b3 {
                            lf_checker_rt::callee_thiscall!(
                                4, u32, obj, elem, fbuf.as_mut_ptr() as u32
                            );
                            let d0 = sub(rdf(esi + ACC_POS), f32::from_bits(fbuf[0]));
                            let d1 = sub(rdf(esi + ACC_POS + 4), f32::from_bits(fbuf[1]));
                            let d2 = sub(rdf(esi + ACC_POS + 8), f32::from_bits(fbuf[2]));
                            let mut nbuf = [d0.to_bits(), d1.to_bits(), d2.to_bits()];
                            lf_checker_rt::callee_thiscall!(6, u32, nbuf.as_mut_ptr() as u32);
                            let g0 = f32::from_bits(nbuf[0]);
                            let g1 = f32::from_bits(nbuf[1]);
                            let g2 = f32::from_bits(nbuf[2]);
                            let t0 = mul(rdf(esi + ACC_DIR + 4), g1);
                            let t1 = mul(rdf(esi + ACC_DIR), g0);
                            let dot = add(add(t1, t0), mul(rdf(esi + ACC_DIR + 8), g2));
                            let mut store = dot > f32::from_bits(lf_checker_rt::global::<u32>(DOT_LIMIT_VA).read());
                            if !store && !below_eq(dot, 0.0) {
                                let ok: u32 = lf_checker_rt::callee_thiscall!(
                                    5,
                                    u32,
                                    a0,
                                    elem.wrapping_add(ELEM_PROBE_OFF)
                                );
                                store = ok & 0xff != 0;
                            }
                            if store {
                                let count = rd32(esi + ACC_COUNT);
                                wr32(
                                    esi + count.wrapping_mul(4) + ACC_CLASS,
                                    rd16(elem + ELEM_CLASS_OFF) & 0xf,
                                );
                                let fv: f32 = lf_checker_rt::callee_stdcall!(
                                    3,
                                    f32,
                                    obj,
                                    magic_quot(elem.wrapping_sub(table))
                                );
                                wrf(esi + count.wrapping_mul(4) + ACC_SCORE, fv);
                                let dst = esi
                                    + count
                                        .wrapping_add(ACC_VEC_BASE)
                                        .wrapping_mul(ACC_VEC_STRIDE);
                                wr32(dst, fbuf[0]);
                                wr32(dst + 4, fbuf[1]);
                                wr32(dst + 8, fbuf[2]);
                                wr32(dst + 0xc, fbuf[3]);
                                let grown = rd32(esi + ACC_COUNT).wrapping_add(1);
                                wr32(esi + ACC_COUNT, grown);
                                if grown as i32 >= MAX_ROWS {
                                    return 0;
                                }
                                let r: u32 = lf_checker_rt::callee_thiscall!(
                                    7, u32, this, a0, esi, obj, elem
                                );
                                if r & 0xff == 0 {
                                    return 0;
                                }
                            }
                        }
                    }
                }
            }
            let _ = deep;
            counter = counter.wrapping_add(1);
            let total = (rd32(a3) >> 0x15) & 0xf;
            if counter >= total {
                return 1;
            }
        }
    }
});
