// original: 0x00a1cbe0 cam_param_blend_snap (proposed)

/// Blends one camera parameter toward a requested value, or snaps it.
///
/// `this` points to a record with a mode word at `+MODE_OFF`, a flag
/// byte at `+FLAG_OFF` and the parameter float at `+VAL_OFF`. `p` points
/// to the requested float. When the mode is zero or the hold bit is set,
/// the parameter is snapped to the request (both end up holding the same
/// bits). Otherwise the parameter moves one exponential-smoothing step,
/// `cur + (req - cur) * BLEND_K` evaluated as `((req - cur) * K) + cur`
/// in that operation order, and the request slot is overwritten with the
/// new parameter value. Returns nothing.
///
/// Original: 0x00a1cbe0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00a1cbe0(this: u32, p: u32) -> u32 {
    unsafe {
        const MODE_OFF: u32 = 0x130;
        const FLAG_OFF: u32 = 0x38c;
        const VAL_OFF: u32 = 0x2e4;
        const HOLD_BIT: u8 = 8;
        const BLEND_K: f32 = f32::from_bits(0x3eccc_ccd); // 0.4
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
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        if rd32(this + MODE_OFF) == 0 || ((this + FLAG_OFF) as *const u8).read() & HOLD_BIT != 0 {
            let v = rd32(p);
            wr32(this + VAL_OFF, v);
            wr32(p, v);
            return 0;
        }
        let cur = f32::from_bits(rd32(this + VAL_OFF));
        let req = f32::from_bits(rd32(p));
        let next = add(mul(sub(req, cur), BLEND_K), cur);
        wr32(this + VAL_OFF, next.to_bits());
        wr32(p, rd32(this + VAL_OFF));
        0
    }
});
