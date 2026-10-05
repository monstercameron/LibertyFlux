// original: 0x008DCB00 CClearRenderTargetDC::vf1

/// CClearRenderTargetDC::vf1 (draw-command virtual slot 1).
///
/// Passes the clear colour (bytes at +0x14/+0x15/+0x16), depth at +0x0c
/// and target id at +0x08/+0x10 to the clear routine.
///
/// Original: 0x008DCB00 (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_008dcb00(this: u32) -> u32 {
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
        lf_checker_rt::callee_cdecl!(1, u32, rd8(this + 0x14) as u32, rd32(this + 0x8), rd8(this + 0x15) as u32, rd32(this + 0xc), rd8(this + 0x16) as u32, rd32(this + 0x10));
        0
    }
});
