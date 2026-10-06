// original: 0x008eb000 tri_classify_by_shape (proposed)

/// Classify a candidate triangle list against the shapes stored in `this`,
/// writing a status code and a shape field through the out-pointers.
///
/// `this` points to an object whose word array at `+0x804` holds pointers to
/// element blocks; each candidate word packs an element index in the low 16
/// bits (unsigned) and a sub-index in the high 16 bits selecting a 32-byte
/// lane within the block. `a0` is carried to one callee but never decides
/// anything. `out0` receives the status (0 preset, then 1, 2, 3 or 4) and
/// `out1` receives the word at `+0x0c` of the selected element, or 0.
///
/// Behaviour: two setup callees fill a 15-word candidate array and two
/// selector words on the frame; a selector whose low 16 bits equal 0xffff
/// (compared as raw bits) exits early. A table lookup through the global
/// pointer table at `TABLE`, indexed by the first selector masked to 16 bits
/// (unsigned), exits with 4 when the flag byte's low three bits are clear.
/// Otherwise the second selector, compared SIGNED against 9 and shifted down
/// by 3, bounds a loop over overlapping 4-word windows of the candidate
/// array. Each iteration walks four element pointers, loads one signed
/// 16-bit triple from `+0x14/+0x16/+0x18` of each (x and y scaled by `CA`,
/// z by `CB`), forms three edge vectors, normalises each with a guarded
/// reciprocal length (an exactly-zero squared length yields 0, anything else
/// including NaN yields `ONE/sqrt`), and combines them into one score `R`.
/// `R` below `T1` exits with 1, above `T2` exits with 2; otherwise a per-shape
/// callee returning a SIGNED value exits with 3 when it reaches 4. Falling
/// off the loop leaves the preset 0.
///
/// Float operation order is the original's; every arithmetic op pins its
/// operands with `black_box`. The `ucomiss/lahf/test/jp` idiom is `x == 0.0`
/// (equality only; NaN takes the divide path on both sides). Integer edge
/// cases: `cmovg` and the final `cmp/jge` are signed; the table index, the
/// element index and the flag test are unsigned; the 0xffff checks are
/// bit equality on the low half.
///
/// Original: 0x008eb000 (thiscall, three stack words; returns nothing, the
/// value left in eax is whatever the last call or load produced).
lf_checker_rt::export!(thiscall, rw_008eb000(this: u32, a0: u32, out0: u32, out1: u32) -> u32 {
    unsafe {
        const ELT_BASE: u32 = 0x804;
        const ELT_X: u32 = 0x14;
        const ELT_Y: u32 = 0x16;
        const ELT_Z: u32 = 0x18;
        const ELT_FIELD: u32 = 0x0c;
        const LANE: u32 = 32;
        const TABLE: u32 = 0x1178384;
        const CA_AT: u32 = 0xfe87a4;
        const CB_AT: u32 = 0xfe8720;
        const ONE_AT: u32 = 0xfe88e8;
        const T1_AT: u32 = 0xfe8d7c;
        const T2_AT: u32 = 0xfe8830;
        const C100: f32 = f32::from_bits(0x42c8_0000);
        const C10000: f32 = f32::from_bits(0x461c_4000);
        const CAL_LIST: u32 = 1;
        const CAL_FILL3: u32 = 2;
        const CAL_SEL: u32 = 3;
        const CAL_CAND: u32 = 4;
        const CAL_FLAG: u32 = 5;
        const CAL_SHAPE: u32 = 6;
        const CAL_COOKIE: u32 = 7;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16s(a: u32) -> i32 {
            unsafe { (a as *const i16).read_unaligned() as i32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
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
        /// Guarded reciprocal length: exactly zero stays zero, anything else
        /// (including NaN, negative, infinite) takes ONE/sqrt.
        #[inline(always)]
        fn rsqrt0(x: f32, one: f32) -> f32 {
            if x == 0.0 {
                0.0
            } else {
                div(one, core::hint::black_box(x).sqrt())
            }
        }
        #[inline(always)]
        unsafe fn walk(this: u32, w: u32) -> u32 {
            unsafe {
                rd32(this.wrapping_add((w & 0xffff).wrapping_mul(4)).wrapping_add(ELT_BASE))
                    .wrapping_add((w >> 16).wrapping_mul(LANE))
            }
        }

        let ca = rdf(lf_checker_rt::relocated(CA_AT));
        let cb = rdf(lf_checker_rt::relocated(CB_AT));
        let one = rdf(lf_checker_rt::relocated(ONE_AT));
        let t1 = rdf(lf_checker_rt::relocated(T1_AT));
        let t2 = rdf(lf_checker_rt::relocated(T2_AT));

        wr32(out0, 0);
        wr32(out1, 0);
        let mut sel0: u32 = 0xffff_ffff;
        let mut sel1: u32 = 0xffff_ffff;
        let mut scratch50: f32 = 0.0;
        let mut slot6f: u32 = 0;
        let mut cand: [u32; 15] = [0xffff_ffff; 15];
        let mut tmp3: [u32; 3] = [0; 3];

        // Note: the setup callee observes ecx = out1 (leftover from zeroing
        // the out-pointers), not `this`; the checker compares ecx for thiscalls.
        let p1: u32 = lf_checker_rt::callee_thiscall!(CAL_LIST, u32, out1);
        let arg50 = rd32(p1.wrapping_add(0x20)).wrapping_add(0x10);
        let r2a: u32 = lf_checker_rt::callee_cdecl!(CAL_FILL3, u32, tmp3.as_mut_ptr() as u32);
        let _r3: u32 = lf_checker_rt::callee_thiscall!(
            CAL_SEL, u32, this, r2a, arg50,
            &mut sel0 as *mut u32 as u32, &mut sel1 as *mut u32 as u32,
            0, &mut scratch50 as *mut f32 as u32, &mut slot6f as *mut u32 as u32
        );
        let edi = sel0;
        sel0 = edi & 0xffff;
        if sel0 == 0xffff {
            let _c: u32 = lf_checker_rt::callee_cdecl!(CAL_COOKIE, u32,);
            return 0;
        }
        let esi = sel1;
        if (esi & 0xffff) == 0xffff {
            let _c: u32 = lf_checker_rt::callee_cdecl!(CAL_COOKIE, u32,);
            return 0;
        }

        let r2b: u32 = lf_checker_rt::callee_cdecl!(CAL_FILL3, u32, tmp3.as_mut_ptr() as u32);
        let _r4: u32 = lf_checker_rt::callee_thiscall!(
            CAL_CAND, u32, this, r2b, esi, a0, cand.as_mut_ptr() as u32,
            &mut sel1 as *mut u32 as u32, 0x10, 0, C100.to_bits(), 0,
            C10000.to_bits(), 1, edi, 0, 0, 0, 0, 0, 0
        );
        let r5: u32 = lf_checker_rt::callee_thiscall!(CAL_FLAG, u32, this, edi, esi);
        let entry = rd32(
            lf_checker_rt::relocated(TABLE).wrapping_add(sel0.wrapping_mul(4)),
        );
        if rd8(entry.wrapping_add(r5.wrapping_mul(8)).wrapping_add(5)) & 7 == 0 {
            wr32(out0, 4);
            let _c: u32 = lf_checker_rt::callee_cdecl!(CAL_COOKIE, u32,);
            return 0;
        }

        let mut count = sel1 as i32;
        if count > 9 {
            count = 9;
        }
        count -= 3;
        if count <= 0 {
            let _c: u32 = lf_checker_rt::callee_cdecl!(CAL_COOKIE, u32,);
            return 0;
        }
        let mut i = 0i32;
        while i < count {
            let j = i as usize;
            let pa = walk(this, cand[j]);
            let pb = walk(this, cand[j + 1]);
            let pc = walk(this, cand[j + 2]);
            let pd = walk(this, cand[j + 3]);
            // Scaled triples: x/y by CA, z by CB.
            let ax = mul(rd16s(pa.wrapping_add(ELT_X)) as f32, ca);
            let ay = mul(rd16s(pa.wrapping_add(ELT_Y)) as f32, ca);
            let az = mul(rd16s(pa.wrapping_add(ELT_Z)) as f32, cb);
            let bx = mul(rd16s(pb.wrapping_add(ELT_X)) as f32, ca);
            let by = mul(rd16s(pb.wrapping_add(ELT_Y)) as f32, ca);
            let bz = mul(rd16s(pb.wrapping_add(ELT_Z)) as f32, cb);
            let cx = mul(rd16s(pc.wrapping_add(ELT_X)) as f32, ca);
            let cy = mul(rd16s(pc.wrapping_add(ELT_Y)) as f32, ca);
            let cz = mul(rd16s(pc.wrapping_add(ELT_Z)) as f32, cb);
            let dx = mul(rd16s(pd.wrapping_add(ELT_X)) as f32, ca);
            let dy = mul(rd16s(pd.wrapping_add(ELT_Y)) as f32, ca);
            let dz = mul(rd16s(pd.wrapping_add(ELT_Z)) as f32, cb);
            // Edge B-A.
            let b_a_y = sub(by, ay);
            let b_a_x = sub(bx, ax);
            let b_a_z = sub(bz, az);
            // Edge C-B.
            let c_b_x = sub(cx, bx);
            let c_b_y = sub(cy, by);
            let c_b_z = sub(cz, bz);
            // Edge D-C.
            let d_c_x = sub(dx, cx);
            let d_c_y = sub(dy, cy);
            let d_c_z = sub(dz, cz);
            // |B-A|^2 as ((y)^2 + (x)^2) + (z)^2.
            let q1y = mul(b_a_y, b_a_y);
            let q1x = mul(b_a_x, b_a_x);
            let q1 = add(add(q1y, q1x), mul(b_a_z, b_a_z));
            let r1 = rsqrt0(q1, one);
            let n1x = mul(b_a_x, r1);
            let n1y = mul(b_a_y, r1);
            // |C-B|^2 as ((y)^2 + (x)^2) + (z)^2.
            let mut m1 = mul(c_b_y, c_b_y);
            m1 = add(m1, mul(c_b_x, c_b_x));
            m1 = add(m1, mul(c_b_z, c_b_z));
            let r2 = rsqrt0(m1, one);
            let n2x = mul(c_b_x, r2);
            let n2y = mul(c_b_y, r2);
            // |D-C|^2 as ((y)^2 + (x)^2) + (z)^2.
            let mut k1 = mul(d_c_y, d_c_y);
            k1 = add(k1, mul(d_c_x, d_c_x));
            k1 = add(k1, mul(d_c_z, d_c_z));
            let r3 = rsqrt0(k1, one);
            // Score: ((dy*r3)*(nx) - (dx*r3)*(ny)) + ((ny*n1x) - (nx*n1y)).
            let u = mul(mul(d_c_y, r3), n2x);
            let v = mul(mul(d_c_x, r3), n2y);
            let w = mul(n2y, n1x);
            let z = mul(n2x, n1y);
            let score = add(sub(u, v), sub(w, z));
            if t1 > score {
                wr32(out1, rd32(pc.wrapping_add(ELT_FIELD)));
                wr32(out0, 1);
                let _c: u32 = lf_checker_rt::callee_cdecl!(CAL_COOKIE, u32,);
                return 0;
            }
            if score > t2 {
                wr32(out1, rd32(pc.wrapping_add(ELT_FIELD)));
                wr32(out0, 2);
                let _c: u32 = lf_checker_rt::callee_cdecl!(CAL_COOKIE, u32,);
                return 0;
            }
            let r6: u32 = lf_checker_rt::callee_thiscall!(CAL_SHAPE, u32, this, pb);
            if (r6 as i32) >= 4 {
                wr32(out1, rd32(pc.wrapping_add(ELT_FIELD)));
                wr32(out0, 3);
                let _c: u32 = lf_checker_rt::callee_cdecl!(CAL_COOKIE, u32,);
                return 0;
            }
            i += 1;
        }
        let _c: u32 = lf_checker_rt::callee_cdecl!(CAL_COOKIE, u32,);
        0
    }
});
