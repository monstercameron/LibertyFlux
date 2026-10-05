// original: 0x008DDD20 CLockRenderTargetDC::vf1

/// CLockRenderTargetDC::vf1 (draw-command virtual slot 1).
///
/// Locks the render target through the global graphics manager: the
/// manager pointer comes from its global, the slot at vtable +0x3c is
/// called with the manager as `this`, the rect words at +0x08/+0x10/
/// +0x14, the constants 0 and 1, and the flags word at +0x0c.
///
/// Original: 0x008DDD20 (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_008ddd20(this: u32) -> u32 {
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
        const MANAGER_GLOBAL: u32 = 0x017F5630;
        const LOCK_SLOT: u32 = 0x3c;
        let mgr = lf_checker_rt::global::<u32>(MANAGER_GLOBAL).read();
        let vtable = (mgr as *const u32).read();
        let target = ((vtable.wrapping_add(LOCK_SLOT)) as *const u32).read();
        let lock: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        lock(mgr, rd32(this + 8), rd32(this + 0x10), rd32(this + 0x14),
            0, 1, rd32(this + 0xc));
        0
    }
});
