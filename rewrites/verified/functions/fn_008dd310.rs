// original: 0x008DD310 CDrawPedPropDC::vf1

/// CDrawPedPropDC::vf1 (draw-command virtual slot 1).
///
/// Draws one ped prop. The shade bytes at +0xef/+0xee are scaled by
/// 1/255 for the shade calls, then the prop call takes the model-table
/// entry for the index at +0xec (forwarded as a value, never
/// dereferenced here), the embedded block at +0xa0, three words and
/// the flag byte at +0xf4.
///
/// Original: 0x008DD310 (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_008dd310(this: u32) -> u32 {
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
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        const MODEL_TABLE: u32 = 0x01295CD8;
        const INV_255: u32 = 0x00FE86E8;
        let inv = lf_checker_rt::global::<f32>(INV_255).read();
        let f0 = mul((rd8(this + 0xef) as f32), inv);
        let f1 = mul((rd8(this + 0xee) as f32), inv);
        lf_checker_rt::callee_cdecl!(1, u32, f1.to_bits());
        lf_checker_rt::callee_cdecl!(2, u32, f0.to_bits());
        let tab = lf_checker_rt::relocated(MODEL_TABLE);
        let entry = (tab.wrapping_add(rd16(this + 0xec).wrapping_mul(4)) as *const u32).read();
        lf_checker_rt::callee_cdecl!(3, u32, entry, this.wrapping_add(0xa0), rd32(this + 0xe4),
            rd32(this + 0xe0), rd32(this + 0xe8), rd32(this + 0xf0),
            rd8(this + 0xf4) as u32);
        0
    }
});
