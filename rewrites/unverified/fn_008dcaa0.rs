// original: 0x008DCAA0 effect_param_forward (proposed)

/// Effect-parameter forward (proposed name).
///
/// Forwards four words and a float to the effect handler stored in
/// the object's function slot at +0x398, together with the embedded
/// parameter block at +0x38c. The call is register-indirect through
/// the object, so the contract plants the stub address there.
///
/// Original: 0x008DCAA0 (thiscall: `this` in ECX, four words, one float).
lf_checker_rt::export!(thiscall, rw_008dcaa0(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, f4: u32) -> u32 {
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
        const PARAMS: u32 = 0x38c;
        const HANDLER_SLOT: u32 = 0x398;
        let target = rd32(this + HANDLER_SLOT);
        let handler: extern "cdecl" fn(u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        handler(this.wrapping_add(PARAMS), a0, a1, a2, a3, f4);
        0
    }
});
