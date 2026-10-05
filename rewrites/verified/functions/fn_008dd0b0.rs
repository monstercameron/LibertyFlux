// original: 0x008DD0B0 CDrawFragTypeDC::vf1

/// CDrawFragTypeDC::vf1 (draw-command virtual slot 1).
///
/// Draws one fragment type. The model index at +0x58 looks up the
/// global model table; a null entry or a model whose detail flag at
/// +0xb4 is clear draws nothing. Otherwise the two shade bytes at
/// +0x5b/0x5a are scaled by 1/255 and passed to the shade calls, the
/// flag byte at +0x5c selects the detail level, the words at
/// +0x54/+0x50 become the current detail globals while the mesh call
/// runs on the embedded mesh at +0x10, and the globals are restored
/// (0xff/0) with the detail level reset afterwards.
///
/// Original: 0x008DD0B0 (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_008dd0b0(this: u32) -> u32 {
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
        const DETAIL_A: u32 = 0x015DBDC4;
        const DETAIL_B: u32 = 0x015DBDC0;
        let tab = lf_checker_rt::relocated(MODEL_TABLE);
        let entry = (tab.wrapping_add(rd16(this + 0x58).wrapping_mul(4)) as *const u32).read();
        if entry == 0 {
            return 0;
        }
        let model = ((entry.wrapping_add(8)) as *const u32).read();
        if ((model.wrapping_add(0xb4)) as *const u32).read() == 0 {
            return 0;
        }
        let inv = lf_checker_rt::global::<f32>(INV_255).read();
        let f0 = mul((rd8(this + 0x5b) as f32), inv);
        let f1 = mul((rd8(this + 0x5a) as f32), inv);
        lf_checker_rt::callee_cdecl!(1, u32, f1.to_bits());
        lf_checker_rt::callee_cdecl!(2, u32, f0.to_bits());
        lf_checker_rt::callee_cdecl!(3, u32, rd8(this + 0x5c) as u32);
        lf_checker_rt::global::<u32>(DETAIL_A).write(rd32(this + 0x54));
        lf_checker_rt::global::<u32>(DETAIL_B).write(rd32(this + 0x50));
        lf_checker_rt::callee_thiscall!(4, u32, model, this.wrapping_add(0x10), 0, 0, 0);
        lf_checker_rt::global::<u32>(DETAIL_A).write(0xff);
        lf_checker_rt::global::<u32>(DETAIL_B).write(0);
        lf_checker_rt::callee_cdecl!(3, u32, 0);
        0
    }
});
