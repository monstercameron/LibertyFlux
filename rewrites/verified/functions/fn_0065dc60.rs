// original: 0x0065DC60 rage::DayLighting::vf1

/// Push two daylight colour vectors and their adjusted forms to the shader.
///
/// `effect` carries the shader view (`+0x18`, the callee's object) and the
/// destination slot (`+0x14`); `var_cache` holds the four cached variable
/// indices (`+4`, `+8`, `+0xc`, `+0x10`). ECX is not read. Twice (once per
/// colour set): the four global colour words are staged; when the global
/// light pointer is non-null its three offset words (`+0x60`, `+0x64`,
/// `+0x68`) are subtracted from the first three (original operand order) and
/// the result is handed to the vector callee, which fills it in place;
/// otherwise the adjusted vector is three zeros plus an unwritten fourth
/// word (zero under the checker's zero stack fill). Each half then issues
/// two setter calls: the staged globals (16 bytes) and the adjusted vector
/// (16 bytes), with constant size/count/stride arguments (0x10, 1, 4).
/// Returns the last setter call's result.
///
/// Original: 0x0065DC60 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_0065dc60(_this: u32, effect: u32, var_cache: u32) -> u32 {
    unsafe {
        const SET_CALLEE: u32 = 2;
        const VEC_CALLEE: u32 = 3;
        const EFFECT_VIEW: u32 = 0x18;
        const EFFECT_DEST: u32 = 0x14;
        const LIGHT_PTR: u32 = 0x017F583C;
        const COLOUR_A: u32 = 0x018DEDE0;
        const COLOUR_B: u32 = 0x018DEE20;
        const LIGHT_OFF_X: u32 = 0x60;
        const LIGHT_OFF_Y: u32 = 0x64;
        const LIGHT_OFF_Z: u32 = 0x68;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        /// Adjusted vector for one colour set: subtracted triple plus the
        /// untouched fourth word, run through the vector callee; or zeros
        /// when no light is present.
        unsafe fn adjusted(colour: u32, light: u32) -> [u32; 4] {
            unsafe {
                let mut vec = [0u32; 4];
                if light != 0 {
                    let x = sub(rdf(colour), rdf(light.wrapping_add(LIGHT_OFF_X)));
                    let y = sub(rdf(colour.wrapping_add(4)), rdf(light.wrapping_add(LIGHT_OFF_Y)));
                    let z = sub(rdf(colour.wrapping_add(8)), rdf(light.wrapping_add(LIGHT_OFF_Z)));
                    vec[0] = x.to_bits();
                    vec[1] = y.to_bits();
                    vec[2] = z.to_bits();
                    vec[3] = rd32(colour.wrapping_add(12));
                    lf_checker_rt::callee_thiscall!(VEC_CALLEE, u32, vec.as_mut_ptr() as u32);
                }
                vec
            }
        }

        let light = rd32(lf_checker_rt::relocated(LIGHT_PTR));
        let view = rd32(effect.wrapping_add(EFFECT_VIEW));
        let dest = effect.wrapping_add(EFFECT_DEST);

        let raw_a = [
            rd32(lf_checker_rt::relocated(COLOUR_A)),
            rd32(lf_checker_rt::relocated(COLOUR_A).wrapping_add(4)),
            rd32(lf_checker_rt::relocated(COLOUR_A).wrapping_add(8)),
            rd32(lf_checker_rt::relocated(COLOUR_A).wrapping_add(12)),
        ];
        let vec_a = adjusted(lf_checker_rt::relocated(COLOUR_A), light);
        lf_checker_rt::callee_thiscall!(
            SET_CALLEE, u32, view, dest, rd32(var_cache.wrapping_add(4)),
            raw_a.as_ptr() as u32, 0x10, 1, 4
        );
        lf_checker_rt::callee_thiscall!(
            SET_CALLEE, u32, view, dest, rd32(var_cache.wrapping_add(8)),
            vec_a.as_ptr() as u32, 0x10, 1, 4
        );

        let raw_b = [
            rd32(lf_checker_rt::relocated(COLOUR_B)),
            rd32(lf_checker_rt::relocated(COLOUR_B).wrapping_add(4)),
            rd32(lf_checker_rt::relocated(COLOUR_B).wrapping_add(8)),
            rd32(lf_checker_rt::relocated(COLOUR_B).wrapping_add(12)),
        ];
        let vec_b = adjusted(lf_checker_rt::relocated(COLOUR_B), light);
        lf_checker_rt::callee_thiscall!(
            SET_CALLEE, u32, view, dest, rd32(var_cache.wrapping_add(0x0c)),
            raw_b.as_ptr() as u32, 0x10, 1, 4
        );
        lf_checker_rt::callee_thiscall!(
            SET_CALLEE, u32, view, dest, rd32(var_cache.wrapping_add(0x10)),
            vec_b.as_ptr() as u32, 0x10, 1, 4
        )
    }
});
