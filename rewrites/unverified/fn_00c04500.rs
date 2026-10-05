// original: 0x00c04500 stream_init_6e (proposed)

/// Initialise a streaming record (tag `0x6e`), normalising a vector in place.
///
/// `this` points to the record. Writes the tag and the shared streaming
/// global, pushes the vector at `a2` through the position callee, then
/// normalises the vector at `v` in place (`v * (1 / sqrt(x*x + y*y + z*z))`;
/// a zero length zeroes it instead) and pushes the normalised vector through
/// the direction callee. Stores `a1` at `+0x18`, the float `f6` at `+0x1c`,
/// the low 6 bits of `a4` into `+0x17`, and at `+0x16` the old bit `0x40`
/// plus the low 6 bits of `a5` plus bit 0 of `a7` shifted to bit 7. Returns
/// `a1` with its low byte replaced by that shifted bit (the original's
/// leftover in `eax`).
///
/// Original: 0x00c04500 (thiscall, seven stack words).
lf_checker_rt::export!(thiscall, rw_00c04500(this: u32, a1: u32, a2: u32, v: u32, a4: u32, a5: u32, f6: u32, a7: u32) -> u32 {
    unsafe {
        const TAG: u8 = 0x6e;
        const GLOBAL_STREAM: u32 = 0x011735a4;
        const CALLEE_POS: u32 = 1;
        const CALLEE_DIR: u32 = 2;
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, val: f32) {
            unsafe { (a as *mut u32).write_unaligned(val.to_bits()) }
        }
        (this as *mut u8).write(TAG);
        let g = (lf_checker_rt::relocated(GLOBAL_STREAM) as *const u32).read_unaligned();
        (this.wrapping_add(4) as *mut u32).write_unaligned(g);
        lf_checker_rt::callee_thiscall!(CALLEE_POS, u32, this, a2);
        let x = rdf(v);
        let y = rdf(v.wrapping_add(4));
        let z = rdf(v.wrapping_add(8));
        let l2 = add(add(mul(x, x), mul(y, y)), mul(z, z));
        let inv = if l2 == 0.0 {
            0.0
        } else {
            core::hint::black_box(1.0f32) / core::hint::black_box(l2.sqrt())
        };
        wrf(v, mul(x, inv));
        wrf(v.wrapping_add(4), mul(y, inv));
        wrf(v.wrapping_add(8), mul(z, inv));
        lf_checker_rt::callee_thiscall!(CALLEE_DIR, u32, this, v);
        (this.wrapping_add(0x18) as *mut u32).write_unaligned(a1);
        let old17 = ((this.wrapping_add(0x17)) as *const u8).read();
        let mut a = old17 ^ ((a4 & 0xff) as u8);
        a &= 0x3f;
        ((this.wrapping_add(0x17)) as *mut u8).write(old17 ^ a);
        let mut cl = ((this.wrapping_add(0x16)) as *const u8).read() & 0x40;
        cl |= ((a5 & 0xff) as u8) & 0x3f;
        let top = (((a7 & 0xff) as u8) << 7) as u32;
        cl |= top as u8;
        wrf(this.wrapping_add(0x1c), f32::from_bits(f6));
        ((this.wrapping_add(0x16)) as *mut u8).write(cl);
        (a1 & 0xffff_ff00) | top
    }
});
