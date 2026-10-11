// original: 0x00a25570 task_aim_solve

/// Solve an aim triple for the task object `obj` into `out_a`/`out_b`/`out_c`.
///
/// Tries the 0x41e task first: when present with a flag object, fills a
/// direction triple through a callee, subtracts the base triple at
/// `[obj + 0x20] + 0x30`, normalises it (a zero vector yields a zero factor,
/// a NaN norm yields NaN), and hands the scaled triple to a direction callee
/// whose four returned floats feed an atan2-style callee (two f64 in XMM0
/// and XMM1, double in XMM0). The float answer fans out through a normalise
/// callee into the three outputs (raw, plus pi/2, minus pi/2), and `[out_a]`
/// is nudged by pi/8 from the flag word's `0x18`/`0x60` bits and the task's
/// `+0x35`/`+0x38` fields. Returns 1.
///
/// Without the first task (or its flag object) the 0x414 task runs the same
/// shape with no final nudge; with neither task the function returns 0.
///
/// The direction helper's two angle inputs are widened from f32 to f64 in
/// XMM0 and XMM1. The angle helper returns the exact f64 bits in its answer.
///
/// Original: 0x00a25570 (stdcall, four stack words).
fn cvtss2sd_bits(value: u32) -> u64 {
    let sign = u64::from(value >> 31) << 63;
    let exponent = (value >> 23) & 0xFF;
    let fraction = u64::from(value & 0x7F_FFFF);
    if exponent == 0xFF {
        if fraction == 0 {
            return sign | 0x7FF0_0000_0000_0000;
        }
        return sign | 0x7FF8_0000_0000_0000 | (fraction << 29);
    }
    if exponent == 0 {
        if fraction == 0 {
            return sign;
        }
        let leading = (fraction as u32).leading_zeros() - 9;
        let normalized_exponent = 896 - leading;
        let normalized_fraction = ((fraction << (leading + 1)) & 0x7F_FFFF) << 29;
        return sign | (u64::from(normalized_exponent) << 52) | normalized_fraction;
    }
    sign | (u64::from(exponent + 896) << 52) | (fraction << 29)
}

fn double_words(value: u64) -> (u32, u32) {
    (value as u32, (value >> 32) as u32)
}

lf_checker_rt::export!(stdcall, rw_00a25570(obj: u32, out_a: u32, out_b: u32, out_c: u32) -> u32 {
    unsafe {
        const ONE: f32 = f32::from_bits(0x3f80_0000);
        const HALF_PI: f32 = f32::from_bits(0x3fc9_0fdb);
        const EIGHTH_PI: f32 = f32::from_bits(0x3ec9_0fdb);
        const SIGN: u32 = 0x8000_0000;
        const FIND: u32 = 1;
        const FILL: u32 = 2;
        const DIR: u32 = 3;
        const ATAN2: u32 = 4;
        const NORM: u32 = 5;

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
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        /// Run the fill/direction/atan2 chain; returns the float answer.
        #[inline(always)]
        unsafe fn solve_chain(obj: u32, d: u32) -> f32 {
            unsafe {
                let mut p = [0u32; 3];
                lf_checker_rt::callee_thiscall!(FILL, u32, d, p.as_mut_ptr() as u32, 0);
                let bx = rd32(obj.wrapping_add(0x20));
                let dx = sub(f32::from_bits(p[1]), rdf(bx.wrapping_add(0x34)));
                let dy = sub(f32::from_bits(p[0]), rdf(bx.wrapping_add(0x30)));
                let dz = sub(f32::from_bits(p[2]), rdf(bx.wrapping_add(0x38)));
                let n2 = add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz));
                let inv = if n2 == 0.0 { 0.0 } else { ONE / n2.sqrt() };
                let mut dir_in = [mul(inv, dy).to_bits(), mul(inv, dx).to_bits(), mul(inv, dz).to_bits()];
                let mut dir_aux = [0u32; 4];
                let rp: u32 = lf_checker_rt::callee_thiscall!(
                    DIR, u32, d, dir_aux.as_mut_ptr() as u32, dir_in.as_mut_ptr() as u32
                );
                let first_angle = cvtss2sd_bits(rd32(rp) ^ SIGN);
                let second_angle = cvtss2sd_bits(rd32(rp.wrapping_add(4)));
                let (first_low, first_high) = double_words(first_angle);
                let (second_low, second_high) = double_words(second_angle);
                let answer_bits: u64 = lf_checker_rt::callee_cdecl!(
                    ATAN2,
                    u64,
                    first_low,
                    first_high,
                    second_low,
                    second_high
                );
                f64::from_bits(answer_bits) as f32
            }
        }

        let mgr = rd32(obj.wrapping_add(0x224)).wrapping_add(0x44);
        let t1: u32 = lf_checker_rt::callee_thiscall!(FIND, u32, mgr, 0x41e);
        if t1 != 0 {
            let d = rd32(obj.wrapping_add(0xd68));
            if d != 0 {
                let f = solve_chain(obj, d);
                let a1: f32 = lf_checker_rt::callee_cdecl!(NORM, f32, f.to_bits());
                wrf(out_a, a1);
                let mut flag: u8 = (rd8(t1.wrapping_add(0x35)) >> 1) & 1;
                if rd32(t1.wrapping_add(0x38)) == 0x10 {
                    flag = if flag != 0 { 0 } else { 1 };
                }
                let a2: f32 = lf_checker_rt::callee_cdecl!(NORM, f32, add(a1, HALF_PI).to_bits());
                wrf(out_c, a2);
                let cur = rdf(out_a);
                let a3: f32 = lf_checker_rt::callee_cdecl!(NORM, f32, sub(cur, HALF_PI).to_bits());
                wrf(out_b, a3);
                let w = rd32(rd32(obj.wrapping_add(0xd68)));
                if w & 0x18 == 0x18 {
                    match (w >> 5) & 3 {
                        0 => {
                            if flag != 0 {
                                wrf(out_a, add(rdf(out_a), EIGHTH_PI));
                            }
                        }
                        1 => {
                            if flag == 0 {
                                wrf(out_a, sub(rdf(out_a), EIGHTH_PI));
                            }
                        }
                        2 => {
                            if flag == 0 {
                                wrf(out_a, sub(rdf(out_a), EIGHTH_PI));
                            } else {
                                wrf(out_a, add(rdf(out_a), EIGHTH_PI));
                            }
                        }
                        _ => {}
                    }
                }
                return 1;
            }
        }
        let mgr2 = rd32(obj.wrapping_add(0x224)).wrapping_add(0x44);
        let t2: u32 = lf_checker_rt::callee_thiscall!(FIND, u32, mgr2, 0x414);
        if t2 == 0 {
            return 0;
        }
        let d = rd32(obj.wrapping_add(0xd68));
        if d == 0 {
            return 0;
        }
        let f = solve_chain(obj, d);
        wrf(out_a, f);
        let a2: f32 = lf_checker_rt::callee_cdecl!(NORM, f32, add(f, HALF_PI).to_bits());
        wrf(out_c, a2);
        let a3: f32 = lf_checker_rt::callee_cdecl!(NORM, f32, sub(rdf(out_a), HALF_PI).to_bits());
        wrf(out_b, a3);
        1
    }
});

