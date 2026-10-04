// original: 0x00db2fe0 ui_rect_cache_refresh (proposed)

/// Refresh a UI element's cached rectangle from its source quad, or rebind
/// the element to a new source.
///
/// `this` is the element, `arg0` an opaque value handed to the notify callee,
/// `arg1` the candidate source (null means "nothing to do"). The element's
/// anchor pointer at `+0x1d4` leads to a flag word: if it is already set, the
/// element is rebound (resolve through the registry global, then store the
/// new handle at `+0x224`); otherwise the notify callee is asked to fill the
/// flag through `arg1`, and when the filled flag yields an object whose
/// source quad (four floats at `+0x214`/`+0x21c`/`+0x218`/`+0x220`) is not
/// all zero, the object's extent getters (virtual slots `+0x20`/`+0x24`) are
/// read and the cached rectangle is recomputed into `+0x204`/`+0x20c`/`+0x208`
/// /`+0x210` as extent-scaled quad plus base offsets from
/// `+0x1f4`/`+0x1fc`/`+0x1f8`/`+0x200`, scaled by the unit global.
///
/// Edge cases: null `arg1` or null anchor returns immediately with no calls
/// and no writes, leaving the original's return register untouched (the
/// rewrite returns 0 there; the return value is unchecked). A zero extent
/// divides by zero and yields infinities, exactly as the original's SSE
/// division does. The all-zero test treats NaN as nonzero and -0.0 as zero.
///
/// Original: 0x00db2fe0 (thiscall, two stack arguments, callee pops 8).
lf_checker_rt::export!(thiscall, rw_00db2fe0(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        const ANCHOR: u32 = 0x1d4;
        const REBIND_SLOT: u32 = 0x224;
        const Q0: u32 = 0x214;
        const Q1: u32 = 0x21c;
        const Q2: u32 = 0x218;
        const Q3: u32 = 0x220;
        const B0: u32 = 0x1f4;
        const B1: u32 = 0x1fc;
        const B2: u32 = 0x1f8;
        const B3: u32 = 0x200;
        const R0: u32 = 0x204;
        const R1: u32 = 0x20c;
        const R2: u32 = 0x208;
        const R3: u32 = 0x210;
        const ENTER_GUARD: u32 = 1;
        const NOTIFY: u32 = 2;
        const FILL_FLAG: u32 = 3;
        const RESOLVE: u32 = 4;
        const BIND: u32 = 5;
        const LEAVE_GUARD: u32 = 6;
        const REGISTRY_GLOBAL: u32 = 0x01bb5554;
        const UNIT_GLOBAL: u32 = 0x00fe88e8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        if arg1 == 0 || rd32(this + ANCHOR) == 0 {
            // Original falls through with the entry return register intact.
            return 0;
        }
        lf_checker_rt::callee_cdecl!(ENTER_GUARD, u32,);
        lf_checker_rt::callee_cdecl!(NOTIFY, u32, arg0);
        if rd32(rd32(this + ANCHOR)) != 0 {
            let anchor: u32 = lf_checker_rt::callee_cdecl!(RESOLVE, u32, arg1, 0);
            let registry = rd32(lf_checker_rt::relocated(REGISTRY_GLOBAL));
            let bound: u32 = lf_checker_rt::callee_thiscall!(BIND, u32, registry, anchor);
            wr32(this + REBIND_SLOT, bound);
            return lf_checker_rt::callee_cdecl!(LEAVE_GUARD, u32,);
        }
        lf_checker_rt::callee_stdcall!(FILL_FLAG, u32, arg1);
        let anchor = rd32(this + ANCHOR);
        if anchor == 0 {
            return lf_checker_rt::callee_cdecl!(LEAVE_GUARD, u32,);
        }
        let obj = rd32(anchor);
        if obj == 0 {
            return lf_checker_rt::callee_cdecl!(LEAVE_GUARD, u32,);
        }
        let q0 = rdf(this + Q0);
        let q1 = rdf(this + Q1);
        let q2 = rdf(this + Q2);
        let q3 = rdf(this + Q3);
        if q0 == 0.0 && q1 == 0.0 && q2 == 0.0 && q3 == 0.0 {
            return lf_checker_rt::callee_cdecl!(LEAVE_GUARD, u32,);
        }
        let width: u32 = {
            let f: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(rd32(obj) + 0x20) as usize);
            f(obj)
        };
        let obj2 = rd32(anchor);
        let height: u32 = {
            let f: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(rd32(obj2) + 0x24) as usize);
            f(obj2)
        };
        let unit = f32::from_bits(rd32(lf_checker_rt::relocated(UNIT_GLOBAL)));
        let hscale = div(unit, (width as i32) as f32);
        let wide = mul(hscale, q0);
        let hnarrow = mul(q1, hscale);
        wrf(this + R0, add(wide, rdf(this + B0)));
        wrf(this + R1, sub(rdf(this + B1), hnarrow));
        let vscale = div(unit, (height as i32) as f32);
        let tall = mul(q2, vscale);
        let vnarrow = mul(q3, vscale);
        wrf(this + R2, add(tall, rdf(this + B2)));
        wrf(this + R3, sub(rdf(this + B3), vnarrow));
        lf_checker_rt::callee_cdecl!(LEAVE_GUARD, u32,)
    }
});
