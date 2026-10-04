// original: 0x00db3340 UITexture::vf82 (merged symbol)

/// Recompute a UI texture's mapping factor from its cached rectangle and the
/// source extent, then hand control to the shared finish routine.
///
/// `this` is the texture object. After a refresh call, the cached rectangle
/// at `+0x204`/`+0x208`/`+0x20c`/`+0x210` is compared against the rest state
/// (zero, zero, unit, unit with the unit global): any difference copies it
/// over the working rectangle at `+0x1f4`/`+0x1f8`/`+0x1fc`/`+0x200`. A zero
/// mode at `+0x1d8` then finishes immediately; otherwise two virtual float
/// getters (slots `+0x8c`, `+0x98`) are read, the working aspect
/// `([1fc]-[1f4])/([200]-[1f8])` is formed, and the source extent getters
/// (slots `+0x20`/`+0x24` on the object behind `+0x1d4`, or the unit constant
/// when there is none) give a ratio that multiplies the aspect. Mode 1
/// divides the first getter by that product and reports it through virtual
/// slot `+0xa0`; mode 2 multiplies the product by the second getter and
/// reports it through slot `+0x94`. Every path ends in a tail jump to the
/// shared finish routine, whose answer is the return value.
///
/// Edge cases: the rest-state test treats NaN as different and -0.0 as zero;
/// zero divisors yield infinities through the original's SSE divisions, bit
/// for bit. Three callees take the address of a frame slot; those addresses
/// are not compared (their words are never read back).
///
/// Original: 0x00db3340 (thiscall, no stack arguments, tail jump).
lf_checker_rt::export!(thiscall, rw_00db3340(this: u32) -> u32 {
    unsafe {
        const WORK: u32 = 0x1f4;
        const CACHE: u32 = 0x204;
        const ANCHOR: u32 = 0x1d4;
        const MODE: u32 = 0x1d8;
        const REFRESH: u32 = 1;
        const FRAME_A: u32 = 4;
        const FRAME_B: u32 = 7;
        const FRAME_C: u32 = 10;
        const FINISH: u32 = 11;
        const SCOPE_CONST_FILE_VA: u32 = 0x017a6658;
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
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
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
        unsafe fn vget(this: u32, vt: u32, slot: u32) -> f32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> f32 =
                    core::mem::transmute(rd32(vt + slot) as usize);
                f(this)
            }
        }
        #[inline(always)]
        unsafe fn vextent(obj: u32, slot: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + slot) as usize);
                f(obj)
            }
        }
        #[inline(always)]
        unsafe fn vreport(this: u32, vt: u32, slot: u32, v: f32) {
            unsafe {
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(vt + slot) as usize);
                f(this, v.to_bits());
            }
        }

        let vt = rd32(this);
        lf_checker_rt::callee_thiscall!(REFRESH, u32, this);
        let unit = f32::from_bits(rd32(lf_checker_rt::relocated(UNIT_GLOBAL)));
        let c0 = rdf(this + CACHE);
        let c1 = rdf(this + CACHE + 12);
        let c2 = rdf(this + CACHE + 8);
        let c3 = rdf(this + CACHE + 4);
        if !(c0 == 0.0 && c1 == unit && c2 == unit && c3 == 0.0) {
            wr32(this + WORK, rd32(this + CACHE));
            wr32(this + WORK + 4, rd32(this + CACHE + 4));
            wr32(this + WORK + 8, rd32(this + CACHE + 8));
            wr32(this + WORK + 12, rd32(this + CACHE + 12));
        }
        let kind = rd32(this + MODE);
        if kind == 0 {
            return lf_checker_rt::callee_thiscall!(FINISH, u32, this);
        }
        let first = vget(this, vt, 0x8c);
        let second = vget(this, vt, 0x98);
        let aspect = div(
            sub(rdf(this + WORK + 8), rdf(this + WORK)),
            sub(rdf(this + WORK + 12), rdf(this + WORK + 4)),
        );
        let mut scratch: u32 = 0;
        let frame = &mut scratch as *mut u32 as u32;
        lf_checker_rt::callee_thiscall!(
            FRAME_A,
            u32,
            frame,
            lf_checker_rt::relocated(SCOPE_CONST_FILE_VA)
        );
        let anchor = rd32(this + ANCHOR);
        let obj = if anchor == 0 { 0 } else { rd32(anchor) };
        let mut factor = unit;
        if obj != 0 {
            let wide = vextent(obj, 0x20);
            let high = vextent(obj, 0x24);
            factor = div((wide as i32) as f32, (high as i32) as f32);
        }
        lf_checker_rt::callee_thiscall!(FRAME_B, u32, frame);
        if kind == 1 {
            vreport(this, vt, 0xa0, div(first, mul(factor, aspect)));
            lf_checker_rt::callee_thiscall!(FRAME_C, u32, frame);
            return lf_checker_rt::callee_thiscall!(FINISH, u32, this);
        }
        if kind == 2 {
            vreport(this, vt, 0x94, mul(mul(factor, aspect), second));
        }
        lf_checker_rt::callee_thiscall!(FRAME_C, u32, frame);
        lf_checker_rt::callee_thiscall!(FINISH, u32, this)
    }
});

