// original: 0x0065D0A0 rage::AtmosphericScattering::vf1

/// Push atmospheric-scattering parameters and sun terms to the shader.
///
/// `params` (ECX) is the scattering parameter block, `effect` carries the
/// shader view (`+0x18`) and destination slot (`+0x14`), `var_cache` the
/// cached variable indices. Two params words (`+0x10`, `+0x14`) go out as
/// single words, a dword (`+0x68`), and one global table entry indexed by
/// the byte at `+0x6c`. An angle is then formed from `+0x10` (scaled by
/// the rate, base and turn globals, in that order) and run through two
/// scalar math callees; both results plus a zero word and an unwritten
/// fourth word (zero under the checker's zero stack fill) go out as a
/// 16-byte vector. The second math callee is polled twice more on the
/// same angle; each answer is scaled by six, shifted by one, clamped to
/// [0, 1] (NaN passes through, as the original's ordered-compare jumps
/// do), and sent as one minus the clamped value, the first also multiplied
/// by `+0x18`. Returns the last call's result.
///
/// Original: 0x0065D0A0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_0065d0a0(params: u32, effect: u32, var_cache: u32) -> u32 {
    unsafe {
        const SET_WORD: u32 = 2;
        const SET_VEC: u32 = 3;
        const SET_TABLE: u32 = 4;
        const SET_DWORD: u32 = 5;
        const MATH_FIRST: u32 = 6;
        const MATH_SECOND: u32 = 7;
        const EFFECT_VIEW: u32 = 0x18;
        const EFFECT_DEST: u32 = 0x14;
        const ANGLE_RATE: u32 = 0x00FE8768;
        const ANGLE_BASE: u32 = 0x00FE87E4;
        const ANGLE_TURN: u32 = 0x00FE8AEC;
        const SUN_SCALE: u32 = 0x00FE8AE0;
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
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        let view = rd32(effect.wrapping_add(EFFECT_VIEW));
        let dest = effect.wrapping_add(EFFECT_DEST);
        let word0 = rd32(params.wrapping_add(0x10));
        lf_checker_rt::callee_thiscall!(
            SET_WORD, u32, view, dest, rd32(var_cache.wrapping_add(4)),
            &word0 as *const u32 as u32, 4, 1, 2
        );
        let word1 = rd32(params.wrapping_add(0x14));
        lf_checker_rt::callee_thiscall!(
            SET_WORD, u32, view, dest, rd32(var_cache.wrapping_add(8)),
            &word1 as *const u32 as u32, 4, 1, 2
        );
        lf_checker_rt::callee_thiscall!(
            SET_DWORD, u32, view, dest, rd32(var_cache.wrapping_add(0x14)),
            rd32(params.wrapping_add(0x68))
        );
        let mode = lf_checker_rt::relocated(MODE_TABLE)
            .wrapping_add((rd8(params.wrapping_add(0x6c)) as u32).wrapping_mul(4));
        lf_checker_rt::callee_thiscall!(
            SET_TABLE, u32, view, dest, rd32(var_cache.wrapping_add(0x1c)),
            mode, 4, 1, 7
        );

        let angle = mul(
            add(
                mul(rdf(params.wrapping_add(0x10)), rdf(lf_checker_rt::relocated(ANGLE_RATE))),
                rdf(lf_checker_rt::relocated(ANGLE_BASE)),
            ),
            rdf(lf_checker_rt::relocated(ANGLE_TURN)),
        );
        let first: f32 = lf_checker_rt::callee_cdecl!(MATH_FIRST, f32, angle.to_bits());
        let second: f32 = lf_checker_rt::callee_cdecl!(MATH_SECOND, f32, angle.to_bits());
        let terms = [first.to_bits(), second.to_bits(), 0, 0];
        lf_checker_rt::callee_thiscall!(
            SET_VEC, u32, view, dest, rd32(var_cache.wrapping_add(0x20)),
            terms.as_ptr() as u32, 0x10, 1, 4
        );

        let one = rdf(lf_checker_rt::relocated(ONE));
        let scale = rdf(lf_checker_rt::relocated(SUN_SCALE));
        let a: f32 = lf_checker_rt::callee_cdecl!(MATH_SECOND, f32, angle.to_bits());
        let mut c0 = add(mul(a, scale), one);
        if 0.0 > c0 {
            c0 = 0.0;
        }
        if c0 > one {
            c0 = one;
        }
        let k0 = mul(sub(one, c0), rdf(params.wrapping_add(0x18)));
        let word2 = k0.to_bits();
        lf_checker_rt::callee_thiscall!(
            SET_WORD, u32, view, dest, rd32(var_cache.wrapping_add(0x0c)),
            &word2 as *const u32 as u32, 4, 1, 2
        );

        let b: f32 = lf_checker_rt::callee_cdecl!(MATH_SECOND, f32, angle.to_bits());
        let mut c1 = add(mul(b, scale), one);
        if 0.0 > c1 {
            c1 = 0.0;
        }
        if c1 > one {
            c1 = one;
        }
        let k1 = sub(one, c1);
        let word3 = k1.to_bits();
        lf_checker_rt::callee_thiscall!(
            SET_WORD, u32, view, dest, rd32(var_cache.wrapping_add(0x24)),
            &word3 as *const u32 as u32, 4, 1, 2
        )
    }
});
