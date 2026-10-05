// original: 0x0061B080 net_cost_model_a

/// Cost-model lookup over a day count and a mode selector.
///
/// `day` is a day count, `mode` selects the adjustment, `base` is the
/// starting cost. When `mode` is 2 the cost gains a fixed bias; above 2
/// (signed) it gains `(mode-3)*30.6+0.5` truncated toward zero, computed
/// in float32 exactly as the original (signed convert, multiply, add,
/// truncate with the x86 out-of-range result, not saturation); below 2
/// it is unchanged. `day-0x76C` then runs through a chain of small
/// signed divisions and remainders (19, 30, 177, 24, with a
/// multiply-high step), the adjusted cost is folded in scaled by 6, and
/// the final remainder mod 24 is returned. All arithmetic wraps.
/// Original: 0x0061B080 (cdecl, three stack words).
lf_checker_rt::export!(cdecl, rw_0061B080(day: u32, mode: u32, base: u32) -> u32 {
    unsafe {
        use core::arch::x86::*;
        const MODE_BIAS: u32 = 0x1F;
        const MUL_CONST: u32 = 0x00FE8B4C;
        const ADD_CONST: u32 = 0x00FE8830;
        const DAY_EPOCH: u32 = 0x76C;
        let mut cost = base;
        if mode == 2 {
            cost = cost.wrapping_add(MODE_BIAS);
        } else if (mode as i32) > 2 {
            let scaled = _mm_cvtepi32_ps(_mm_set1_epi32(mode.wrapping_sub(3) as i32));
            let scaled = _mm_mul_ss(
                scaled,
                _mm_load_ss(lf_checker_rt::relocated(MUL_CONST) as *const f32),
            );
            let scaled = _mm_add_ss(
                scaled,
                _mm_load_ss(lf_checker_rt::relocated(ADD_CONST) as *const f32),
            );
            let adjust = _mm_cvttss_si32(_mm_set_ss(_mm_cvtss_f32(scaled)));
            cost = cost.wrapping_add(adjust as u32);
        }
        let phase = (day.wrapping_sub(DAY_EPOCH) as i32) % 19;
        let sub = (phase * 11 + 0x1D) % 30;
        let mut slot = sub;
        if slot == 0x19 || slot == 0x18 {
            slot += 1;
        }
        let mixed = (slot as u32).wrapping_add(cost);
        let grown = mixed.wrapping_mul(3).wrapping_mul(2).wrapping_add(5);
        let resid = (grown as i32) % 0xB1;
        let wide = (0x2E8BA2E9u32 as i32 as i64) * (resid as i64);
        let quot = ((wide >> 32) as i32) >> 2;
        let fixed = (quot as u32).wrapping_shr(31).wrapping_add(0xC).wrapping_add(quot as u32);
        ((fixed as i32) % 0x18) as u32
    }
});
