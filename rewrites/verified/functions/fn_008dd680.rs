// original: 0x008DD680 CDrawRadarMapSectionDC::vf1

/// CDrawRadarMapSectionDC::vf1 (draw-command virtual slot 1).
///
/// Draws one radar-map section: texture setup selected by the handle
/// at +0x70 (zero means default setup plus the shared-relay restore
/// at the end), a fixed four-iteration strip loop reading vertex
/// pairs walking down from +0x68/+0x44, then teardown. Each strip
/// call takes five vertex floats, three zero words, -1.0 and the
/// colour word at +0x74.
///
/// Original: 0x008DD680 (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_008dd680(this: u32) -> u32 {
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
        const TEX: u32 = 0x70;
        const COLOUR: u32 = 0x74;
        const NEG_ONE: u32 = 0xbf800000;
        lf_checker_rt::callee_cdecl!(1, u32, rd32(this + TEX));
        if rd32(this + TEX) == 0 {
            lf_checker_rt::callee_cdecl!(2, u32,);
        }
        lf_checker_rt::callee_cdecl!(3, u32, 5, 4);
        let mut i: u32 = 0;
        while i < 4 {
            let bx = this.wrapping_add(0x68).wrapping_sub(i.wrapping_mul(8));
            let di = this.wrapping_add(0x44).wrapping_sub(i.wrapping_mul(0x10));
            lf_checker_rt::callee_cdecl!(4, u32, rd32(di.wrapping_sub(4)), rd32(di), rd32(di.wrapping_add(4)),
                0, 0, NEG_ONE, rd32(this + COLOUR), rd32(bx), rd32(bx.wrapping_add(4)));
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
