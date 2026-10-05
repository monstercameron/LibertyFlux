// original: 0x008DD5A0 CDrawPtxEffectInst::vf1

/// CDrawPtxEffectInst::vf1 (draw-command virtual slot 1).
///
/// Draws one particle-effect instance. The manager at +0x08 gates on
/// two flag bytes; four globals (wireframe flag, two id words, render
/// mode) select an early parameter path, otherwise two effect bits at
/// +0x254 gate. The parameter call takes the two global floats by
/// frame pointer (compared by pointed-to value), the band index comes
/// from a strided global table, and the instance call runs on the
/// global effect object with the scale calls around it.
///
/// Original: 0x008DD5A0 (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_008dd5a0(this: u32) -> u32 {
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
        const WIREFRAME: u32 = 0x011F7060;
        const CUR_ID: u32 = 0x012088B4;
        const WANT_ID: u32 = 0x00F1C040;
        const RENDER_MODE: u32 = 0x01037720;
        const PARAM_A: u32 = 0x0139C234;
        const PARAM_B: u32 = 0x0139C238;
        const BAND_SEL: u32 = 0x01174794;
        const BAND_TAB: u32 = 0x015E8994;
        const FX_OBJ: u32 = 0x01BB6674;
        const ONE: u32 = 0x3f800000;
        let mgr = rd32(this + 8);
        if (mgr.wrapping_add(0x1d1) as *const u8).read() != 0
            && (mgr.wrapping_add(0x1d0) as *const u8).read() == 0
        {
            return 0;
        }
        let direct = lf_checker_rt::global::<u32>(WIREFRAME).read() != 1
            && lf_checker_rt::global::<u32>(CUR_ID).read()
                == lf_checker_rt::global::<u32>(WANT_ID).read()
            && lf_checker_rt::global::<u32>(RENDER_MODE).read() != 0x12;
        if !direct {
            let bits = (mgr.wrapping_add(0x254) as *const u32).read();
            if bits & 0x4 == 0 {
                return 0;
            }
            if bits & 0x20 == 0 {
                return 0;
            }
        }
        let mut params = [lf_checker_rt::global::<u32>(PARAM_A).read(),
            lf_checker_rt::global::<u32>(PARAM_B).read()];
        lf_checker_rt::callee_cdecl!(1, u32, params.as_mut_ptr() as u32);
        let n = lf_checker_rt::global::<u32>(BAND_SEL).read();
        let band = (lf_checker_rt::relocated(BAND_TAB)
            .wrapping_add(n.wrapping_mul(0x210)) as *const u32)
            .read();
        lf_checker_rt::callee_cdecl!(2, u32, band);
        let fx = lf_checker_rt::global::<u32>(FX_OBJ).read();
        lf_checker_rt::callee_thiscall!(3, u32, fx, mgr);
        lf_checker_rt::callee_cdecl!(2, u32, ONE);
        0
    }
});
