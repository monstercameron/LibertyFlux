// original: 0x0065D7A0 rage::IntervalShadows::vf1

/// Push interval-shadow parameters and cached settings to the shader.
///
/// `params` (ECX) is the shadow parameter block, `effect` carries the
/// shader view (`+0x18`) and destination slot (`+0x14`), `var_cache` the
/// cached variable indices (`+4` .. `+0x2c`). First an output pair is
/// derived: three params words (`+0x20`, `+0x24`, `+0x28`) are xored with
/// the sign global, and unless the middle one compares equal to zero the
/// other two are scaled by one over its absolute value; the pair is then
/// two dot-product-like accumulations over further params words, in the
/// original's operation order. Eleven setter calls follow: the output pair
/// plus a zero word (16 bytes), two single params words, two 16-byte
/// params ranges passed by heap pointer, four cached dwords, two global
/// table entries indexed by params bytes `+0x79`/`+0x78`, and a final
/// single word. Returns the last call's result.
///
/// Original: 0x0065D7A0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_0065d7a0(params: u32, effect: u32, var_cache: u32) -> u32 {
    unsafe {
        const SET_VEC: u32 = 2;
        const SET_HEAP: u32 = 3;
        const SET_TABLE: u32 = 4;
        const SET_WORD: u32 = 5;
        const SET_DWORD: u32 = 6;
        const EFFECT_VIEW: u32 = 0x18;
        const EFFECT_DEST: u32 = 0x14;
        const SIGN_BITS: u32 = 0x00FE8FA0;
        const ABS_MASK: u32 = 0x00FE8F80;
        const ONE: u32 = 0x00FE88E8;
        const MODE_TABLE: u32 = 0x0106B550;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        let sign = rd32(lf_checker_rt::relocated(SIGN_BITS));
        let mut x = f32::from_bits(rdf(params.wrapping_add(0x24)).to_bits() ^ sign);
        let mut y = f32::from_bits(rdf(params.wrapping_add(0x20)).to_bits() ^ sign);
        let mut z = f32::from_bits(rdf(params.wrapping_add(0x28)).to_bits() ^ sign);
        let mut w = rdf(params.wrapping_add(0x40));
        // The original guards the scaling with an unordered-compare read
        // through LAHF: it is skipped exactly when x compares equal to
        // zero (either sign), and taken for NaN.
        if x != 0.0 {
            let ax = f32::from_bits(
                x.to_bits() & rd32(lf_checker_rt::relocated(ABS_MASK)),
            );
            let q = core::hint::black_box(rdf(lf_checker_rt::relocated(ONE)))
                / core::hint::black_box(ax);
            z = mul(z, q);
            y = mul(y, q);
        }
        let mut out0 = rdf(params.wrapping_add(0x30));
        let mut out1 = rdf(params.wrapping_add(0x34));
        let t = mul(rdf(params.wrapping_add(0x50)), z);
        out0 = mul(out0, y);
        w = mul(w, 0.0);
        out0 = add(out0, w);
        out1 = mul(out1, y);
        out0 = add(out0, t);
        let u = mul(rdf(params.wrapping_add(0x44)), 0.0);
        out1 = add(out1, u);
        let v = mul(rdf(params.wrapping_add(0x54)), z);
        out1 = add(out1, v);

        let view = rd32(effect.wrapping_add(EFFECT_VIEW));
        let dest = effect.wrapping_add(EFFECT_DEST);
        let pair = [out0.to_bits(), out1.to_bits(), 0, 0];
        lf_checker_rt::callee_thiscall!(
            SET_VEC, u32, view, dest, rd32(var_cache.wrapping_add(4)),
            pair.as_ptr() as u32, 0x10, 1, 4
        );
        let single0 = rd32(params.wrapping_add(0x70));
        lf_checker_rt::callee_thiscall!(
            SET_WORD, u32, view, dest, rd32(var_cache.wrapping_add(8)),
            &single0 as *const u32 as u32, 4, 1, 2
        );
        lf_checker_rt::callee_thiscall!(
            SET_HEAP, u32, view, dest, rd32(var_cache.wrapping_add(0x0c)),
            params.wrapping_add(0x30), 0x10, 1, 5
        );
        lf_checker_rt::callee_thiscall!(
            SET_HEAP, u32, view, dest, rd32(var_cache.wrapping_add(0x10)),
            params.wrapping_add(0x40), 0x10, 1, 5
        );
        lf_checker_rt::callee_thiscall!(
            SET_DWORD, u32, view, dest, rd32(var_cache.wrapping_add(0x14)),
            rd32(params.wrapping_add(0x14))
        );
        lf_checker_rt::callee_thiscall!(
            SET_DWORD, u32, view, dest, rd32(var_cache.wrapping_add(0x18)),
            rd32(params.wrapping_add(0x10))
        );
        lf_checker_rt::callee_thiscall!(
            SET_DWORD, u32, view, dest, rd32(var_cache.wrapping_add(0x1c)),
            rd32(params.wrapping_add(0x18))
        );
        lf_checker_rt::callee_thiscall!(
            SET_DWORD, u32, view, dest, rd32(var_cache.wrapping_add(0x20)),
            rd32(params.wrapping_add(0x1c))
        );
        let mode_hi = lf_checker_rt::relocated(MODE_TABLE)
            .wrapping_add((rd8(params.wrapping_add(0x79)) as u32).wrapping_mul(4));
        lf_checker_rt::callee_thiscall!(
            SET_TABLE, u32, view, dest, rd32(var_cache.wrapping_add(0x24)),
            mode_hi, 4, 1, 7
        );
        let mode_lo = lf_checker_rt::relocated(MODE_TABLE)
            .wrapping_add((rd8(params.wrapping_add(0x78)) as u32).wrapping_mul(4));
        lf_checker_rt::callee_thiscall!(
            SET_TABLE, u32, view, dest, rd32(var_cache.wrapping_add(0x28)),
            mode_lo, 4, 1, 7
        );
        let single1 = rd32(params.wrapping_add(0x74));
        lf_checker_rt::callee_thiscall!(
            SET_WORD, u32, view, dest, rd32(var_cache.wrapping_add(0x2c)),
            &single1 as *const u32 as u32, 4, 1, 2
        )
    }
});
