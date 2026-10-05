// original: 0x008DDC40 CGrcState_SetLightingMode::vf1

/// CGrcState_SetLightingMode::vf1 (draw-command virtual slot 1).
///
/// Sets graphics state 1 (lighting mode) to the word at +0x08 clamped
/// to the global cap (signed minimum: values above the cap become the
/// cap, `cmovg` is a signed comparison).
///
/// Original: 0x008DDC40 (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_008ddc40(this: u32) -> u32 {
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
        const MODE: u32 = 0x08;
        const CAP_GLOBAL: u32 = 0x0106B310;
        let cap = lf_checker_rt::global::<i32>(CAP_GLOBAL).read();
        let mode = rd32(this + MODE) as i32;
        let clamped = if mode > cap { cap } else { mode };
        lf_checker_rt::callee_cdecl!(1, u32, 1, clamped as u32);
        0
    }
});
