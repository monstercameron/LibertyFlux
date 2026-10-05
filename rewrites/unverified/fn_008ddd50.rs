// original: 0x008DDD50 CRenderFontBufferDC::vf1

/// CRenderFontBufferDC::vf1 (draw-command virtual slot 1).
///
/// Renders a font buffer of `len` bytes (at +0x08; empty buffers
/// return at once). The length is rounded up to 16 bytes for the
/// allocator call on the global allocator object, then converted to a
/// pixel width: `len` as an unsigned 64-bit float scaled by -0.5 and
/// truncated toward zero, doubled, and subtracted from the allocated
/// address to form the two arguments of the blit call. The float chain
/// is int-to-double conversion plus a 0-or-2^32 table adjust, narrowing
/// to float, a multiply and truncation to int, each exact or
/// round-to-nearest-even, reproduced in the same order.
///
/// Original: 0x008DDD50 (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_008ddd50(this: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        const LEN: u32 = 0x08;
        const ALLOCATOR: u32 = 0x01175C58;
        const ADJUST_TAB: u32 = 0x00FE8F50;
        const SCALE: u32 = 0x00FE8D7C;
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        let len = rd32(this + LEN);
        if len == 0 {
            return 0;
        }
        let aligned = len.wrapping_add(len.wrapping_neg() & 0xf);
        let base = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(ALLOCATOR),
            aligned, 0xffffffffu32);
        let adjust = lf_checker_rt::relocated(ADJUST_TAB);
        let d = (len as i32) as f64
            + ((adjust + (len >> 31) * 8) as *const f64).read_unaligned();
        let f = mul(d as f32, lf_checker_rt::global::<f32>(SCALE).read());
        let n = f as i32;
        let doubled = n.wrapping_add(n);
        lf_checker_rt::callee_cdecl!(2, u32, base, base.wrapping_sub(doubled as u32));
        0
    }
});
