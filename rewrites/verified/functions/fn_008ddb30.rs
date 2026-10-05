// original: 0x008DDB30 CDrawTriShapeDC::vf1

/// CDrawTriShapeDC::vf1 (draw-command virtual slot 1).
///
/// Draws one triangle shape: texture setup selected by the handle at
/// +0x68 (zero means default setup plus the shared-relay restore at
/// the end), then one strip call per triangle (`count` at +0x70,
/// walking the vertex array up from +0x38), then teardown. Each strip
/// call takes five vertex floats, three zero words, -1.0 and the
/// index word at +0x6c. The list size (171) is 11 short; the body is
/// 182 bytes.
///
/// Original: 0x008DDB30 (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_008ddb30(this: u32) -> u32 {
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
        const TEX: u32 = 0x68;
        const COUNT: u32 = 0x70;
        const INDEX: u32 = 0x6c;
        const DIM: u32 = 0x74;
        const NEG_ONE: u32 = 0xbf800000;
        lf_checker_rt::callee_cdecl!(1, u32, rd32(this + TEX));
        if rd32(this + TEX) == 0 {
            lf_checker_rt::callee_cdecl!(2, u32,);
        }
        lf_checker_rt::callee_cdecl!(3, u32, rd32(this + DIM), rd32(this + COUNT));
        let count = rd32(this + COUNT);
        let mut i: u32 = 0;
        while i < count {
            let di = this.wrapping_add(0x38).wrapping_add(i.wrapping_mul(8));
            lf_checker_rt::callee_cdecl!(4, u32, rd32(di.wrapping_sub(0x30)), rd32(di.wrapping_sub(0x2c)),
                0, 0, 0, NEG_ONE, rd32(this + INDEX), rd32(di),
                rd32(di.wrapping_add(4)));
            i += 1;
        }
        lf_checker_rt::callee_cdecl!(5, u32,);
        if rd32(this + TEX) == 0 {
        let g = lf_checker_rt::global::<u32>(0x011736C8).read();
        lf_checker_rt::callee_thiscall!(6, u32, g);
        lf_checker_rt::callee_fastcall!(7, u32, g, 0);
        }
        0
    }
});
