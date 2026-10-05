// original: 0x008DD8B0 CDrawSLODPedDC::vf1

/// CDrawSLODPedDC::vf1 (draw-command virtual slot 1).
///
/// Draws one low-detail ped. The shade bytes at +0x15/+0x14 are
/// scaled by 1/255 for the shade calls, the allocator call builds the
/// frame block from the words at +0x0c/+0x10, and the draw call takes
/// the block and the words at +0x08/+0x18. Straight line, no table
/// and no branches.
///
/// Original: 0x008DD8B0 (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_008dd8b0(this: u32) -> u32 {
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
        const INV_255: u32 = 0x00FE86E8;
        let inv = lf_checker_rt::global::<f32>(INV_255).read();
        let f0 = mul((rd8(this + 0x15) as f32), inv);
        let f1 = mul((rd8(this + 0x14) as f32), inv);
        lf_checker_rt::callee_cdecl!(1, u32, f1.to_bits());
        lf_checker_rt::callee_cdecl!(2, u32, f0.to_bits());
        let block = lf_checker_rt::callee_cdecl!(4, u32, rd32(this + 0x0c), rd32(this + 0x10), 0, 0);
        lf_checker_rt::callee_cdecl!(5, u32, block, rd32(this + 0x08), rd32(this + 0x18));
        0
    }
});
