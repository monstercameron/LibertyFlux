// original: 0x00a89030 input_ui_clamped_range_query (proposed)

/// Query a clamped range value from the input/UI state object.
///
/// `this` points to the state object (scale at `+0x1454`, parameter block
/// pointer at `+0x1468`, stage word at `+0x146C`). `arg0` is a nullable
/// object pointer consulted for refinement; `vec_a` and `arg2` point at two
/// three-float vectors; the fourth stack word is unread scratch. Returns a
/// float in ST0.
///
/// The base value is the length of (`vec_a` minus `arg2`), scaled by the
/// object's scale and the parameter block's first word. When the stage word
/// reaches 10 and `arg0` is present, the value is refined: if `arg0` selects
/// the detailed kind (bits `0x3C0` of the word at `+0x28` equal `0x80`) with
/// a clear flag at `+0x1304`, a probe callee supplies a threshold, the detail
/// pointer at `+0x20` is initialised on demand through two callees, and the
/// dot of the detail row with (`arg2` minus the anchor) minus the threshold,
/// floored at 0.2, caps the value. A three-entry table picked by the signed
/// word at `+0x2E` through the table at `0x01295CD8` then caps it by the
/// smallest of three half-scaled spans. The tail clamps the value into the
/// parameter block's band (floor at `+8`, ceiling at `+4`).
///
/// Every ordered float test is a strict greater-than in the original, so NaN
/// always takes the fall-through path; the rewrite keeps each test and each
/// arithmetic step in the original's order through exact helpers.
///
/// Original: 0x00A89030 (thiscall, four stack words).
lf_checker_rt::export!(thiscall, rw_00a89030(this: u32, arg0: u32, vec_a: u32, arg2: u32, _a3: u32) -> f32 {
    unsafe {
        const CALLEE_PROBE: u32 = 1;
        const CALLEE_PREP: u32 = 2;
        const CALLEE_DETAIL: u32 = 3;
        const F_SCALE: u32 = 0x1454;
        const F_PARAMS: u32 = 0x1468;
        const F_STAGE: u32 = 0x146c;
        const O_KIND: u32 = 0x28;
        const O_FLAG: u32 = 0x1304;
        const O_DETAIL: u32 = 0x20;
        const O_SEL: u32 = 0x2e;
        const K_FLOOR: u32 = 0x00fe87d0;
        const K_HALF: u32 = 0x00fe8830;
        const TABLE: u32 = 0x01295cd8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn gf(va: u32) -> f32 {
            unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(va))) }
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

        let dx = sub(rdf(vec_a), rdf(arg2));
        let dy = sub(rdf(vec_a.wrapping_add(4)), rdf(arg2.wrapping_add(4)));
        let dz = sub(rdf(vec_a.wrapping_add(8)), rdf(arg2.wrapping_add(8)));
        let params = rd32(this.wrapping_add(F_PARAMS));
        let mut d = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz)).sqrt();
        d = mul(d, rdf(this.wrapping_add(F_SCALE)));
        d = mul(d, rdf(params));
        let stage = rd32(this.wrapping_add(F_STAGE)) as i32;
        if stage >= 10 && arg0 != 0 {
            if rd32(arg0.wrapping_add(O_KIND)) & 0x3c0 == 0x80
                && rd32(arg0.wrapping_add(O_FLAG)) == 0
            {
                let t: f32 =
                    lf_checker_rt::callee_thiscall!(CALLEE_PROBE, f32, this, arg0);
                if rd32(arg0.wrapping_add(O_DETAIL)) == 0 {
                    lf_checker_rt::callee_thiscall!(CALLEE_PREP, u32, arg0);
                    lf_checker_rt::callee_thiscall!(
                        CALLEE_DETAIL,
                        u32,
                        arg0.wrapping_add(0x10),
                        0u32
                    );
                }
                let p = rd32(arg0.wrapping_add(O_DETAIL));
                let q = if p != 0 {
                    p.wrapping_add(0x30)
                } else {
                    arg0.wrapping_add(0x10)
                };
                let dy2 = sub(rdf(arg2.wrapping_add(4)), rdf(q.wrapping_add(4)));
                let dx2 = sub(rdf(arg2), rdf(q));
                let dz2 = sub(rdf(arg2.wrapping_add(8)), rdf(q.wrapping_add(8)));
                let mut s = add(add(mul(rdf(p.wrapping_add(0x24)), dy2), mul(rdf(p.wrapping_add(0x20)), dx2)), mul(rdf(p.wrapping_add(0x28)), dz2));
                s = sub(s, t);
                let floor = gf(K_FLOOR);
                let m1 = if floor > s { floor } else { s };
                if d > m1 {
                    d = m1;
                }
            }
            let sel = (arg0.wrapping_add(O_SEL) as *const u16).read_unaligned() as i16 as i32;
            let entry = rd32(
                lf_checker_rt::relocated(TABLE).wrapping_add((sel as u32).wrapping_mul(4)),
            );
            let half = gf(K_HALF);
            let u = mul(sub(rdf(entry.wrapping_add(0x30)), rdf(entry.wrapping_add(0x20))), half);
            let v = mul(sub(rdf(entry.wrapping_add(0x34)), rdf(entry.wrapping_add(0x24))), half);
            let w = mul(sub(rdf(entry.wrapping_add(0x38)), rdf(entry.wrapping_add(0x28))), half);
            let m2 = if v > u {
                if w > u { u } else { w }
            } else if w > v {
                v
            } else {
                w
            };
            if d > m2 {
                d = m2;
            }
        }
        let params = rd32(this.wrapping_add(F_PARAMS));
        let lo = rdf(params.wrapping_add(8));
        if !(d > lo) {
            d = lo;
        }
        let hi = rdf(params.wrapping_add(4));
        if hi > d {
            d
        } else {
            hi
        }
    }
});
