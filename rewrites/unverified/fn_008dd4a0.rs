// original: 0x008DD4A0 CDrawPlayerDC::vf1

/// CDrawPlayerDC::vf1 (draw-command virtual slot 1).
///
/// Draws the player model. The model index at +0x08 looks up the
/// global model table with the same two pointer shapes as the
/// ped-props draw (a null entry faults on both sides, kept rare).
/// The shade bytes at +0x0b/+0x0a are scaled by 1/255, the allocator
/// call builds the frame block, and the draw call takes the resolved
/// entry, the block, the embedded matrix at +0x20 and the trailing
/// words.
///
/// Original: 0x008DD4A0 (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_008dd4a0(this: u32) -> u32 {
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
        let tab = lf_checker_rt::relocated(MODEL_TABLE);
        let entry = (tab.wrapping_add(rd16(this + 0x08).wrapping_mul(4)) as *const u32).read();
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
        let f0 = mul((rd8(this + 0x0b) as f32), inv);
        let f1 = mul((rd8(this + 0x0a) as f32), inv);
        lf_checker_rt::callee_cdecl!(1, u32, f1.to_bits());
        lf_checker_rt::callee_cdecl!(2, u32, f0.to_bits());
        lf_checker_rt::callee_cdecl!(3, u32, rd8(this + 0x0c) as u32);
        let block = lf_checker_rt::callee_cdecl!(4, u32, rd32(this + 0x398), rd32(this + 0x39c), 0, 0);
        lf_checker_rt::callee_cdecl!(5, u32, entry, block, rd32(this + 0x10), this.wrapping_add(0x20),
            rd32(this + 0x390), rd32(this + 0x3a0), rd32(this + 0x14));
        lf_checker_rt::callee_cdecl!(3, u32, 0);
        0
    }
});
