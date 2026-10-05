// original: 0x008DD3B0 CDrawPedPropsDC::vf1

/// CDrawPedPropsDC::vf1 (draw-command virtual slot 1).
///
/// Draws one ped-props set. The count byte at +0xad gates everything;
/// the model index at +0xa8 looks up the global model table and the
/// entry resolves through one of two pointer shapes (direct detail
/// flag at +0xb4, or an extra hop through +0xc). The shade bytes at
/// +0xab/+0xaa are scaled by 1/255, the allocator call sizes a block
/// from the count, and the props call takes the resolved entry, the
/// block, the embedded matrix at +0x10 and the trailing words.
///
/// Original: 0x008DD3B0 (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_008dd3b0(this: u32) -> u32 {
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
        const ALLOCATOR: u32 = 0x01175C58;
        if rd8(this + 0xad) == 0 {
            return 0;
        }
        let tab = lf_checker_rt::relocated(MODEL_TABLE);
        let entry = (tab.wrapping_add(rd16(this + 0xa8).wrapping_mul(4)) as *const u32).read();
        let direct = (core::hint::black_box(entry).wrapping_add(8) as *const u32).read();
        if direct == 0 {
            let hop = (entry.wrapping_add(0xc) as *const u32).read();
            if hop == 0 {
                return 0;
            }
            if (hop as *const u32).read() == 0 {
                return 0;
            }
        } else if ((direct.wrapping_add(0xb4)) as *const u32).read() == 0 {
            return 0;
        }
        let inv = lf_checker_rt::global::<f32>(INV_255).read();
        let f0 = mul((rd8(this + 0xab) as f32), inv);
        let f1 = mul((rd8(this + 0xaa) as f32), inv);
        lf_checker_rt::callee_cdecl!(1, u32, f1.to_bits());
        lf_checker_rt::callee_cdecl!(2, u32, f0.to_bits());
        lf_checker_rt::callee_cdecl!(3, u32, rd8(this + 0xac) as u32);
        let block = lf_checker_rt::callee_thiscall!(4, u32, lf_checker_rt::relocated(ALLOCATOR),
            (rd8(this + 0xad) as u32).wrapping_shl(6), 0xffffffffu32);
        lf_checker_rt::callee_cdecl!(5, u32, entry, block, this.wrapping_add(0x10), rd32(this + 0xa0),
            rd32(this + 0xb0), rd8(this + 0xb4) as u32);
        lf_checker_rt::callee_cdecl!(3, u32, 0);
        0
    }
});
