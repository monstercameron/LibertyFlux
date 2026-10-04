// original: 0x008acef0 audio_curve_evaluate
/// Evaluates one audio control curve at the given input value.
///
/// The object descriptor selects the behaviour: a zero link word yields 0,
/// mode 4 forwards the raw input to its dedicated evaluator (callee 1),
/// otherwise the input is optionally range-adjusted (below the floor it is
/// raised to the floor and the ceiling skipped; otherwise it is capped at
/// the ceiling) and dispatched by mode: mode 1 reads a stored constant,
/// modes 2-3 and 5-12 forward the adjusted input to per-mode evaluators
/// (callees 2-11), mode 13 runs the polynomial shaper below, mode 14
/// forwards to callee 12, and any other mode yields 0. The result is
/// returned on the x87 stack.
export!(thiscall, rw_008acef0(this: *const u8, arg_bits: u32) -> f64 {
    unsafe {
        if *(this as *const u32) == 0 {
            return 0.0;
        }
        let mode = *this.add(0x24);
        if mode == 4 {
            let r: f32 = callee_thiscall!(1, f32, this as u32, arg_bits);
            return r as f64;
        }
        let mut v = f32::from_bits(arg_bits);
        if *this.add(0x25) != 0 {
            let lo = f32::from_bits(*(this.add(0x1C) as *const u32));
            let hi = f32::from_bits(*(this.add(0x20) as *const u32));
            // Note the if/else shape: when the input is below the floor it
            // is raised to the floor and the ceiling is skipped entirely.
            if lo > v {
                v = lo;
            } else if v > hi {
                v = hi;
            }
        }
        // Jump-table dispatch, transcribed from the pristine table at
        // analysis time (entry 3 is dead: mode 4 returns before the switch).
        const DISPATCH: [u32; 14] = [0, 2, 3, 0, 4, 5, 6, 7, 8, 9, 10, 11, 0, 12];
        let case = mode.wrapping_sub(1);
        if case == 0 {
            return f32::from_bits(*(this.add(4) as *const u32)) as f64;
        }
        if case == 12 {
            return poly_shaper(v) as f64;
        }
        if case > 13 {
            return 0.0;
        }
        let id = DISPATCH[case as usize];
        if id == 0 {
            return 0.0;
        }
        let r: f32 = callee_thiscall!(id, f32, this as u32, v.to_bits());
        r as f64
    }
});

/// Polynomial shaper used by curve mode 13: a scaled-and-shifted input is
/// run through an 8th-order polynomial with saturation clamps on both ends.
/// Inputs at or above the top clamp yield -100.0; inputs whose shifted value
/// is negative yield 0.0.
fn poly_shaper(v: f32) -> f32 {
    unsafe {
        let mut x2 = v - *global::<f32>(0x00E7C9FC);
        x2 *= *global::<f32>(0x00E7C9D8);
        let mut x4 = x2 * x2;
        let mut x0 = x2 * *global::<f32>(0x00E7CA10);
        let mut x1 = x4 * *global::<f32>(0x00E7C9F0);
        x0 -= *global::<f32>(0x00E7C9F8);
        x4 *= x2;
        x1 += x0;
        x0 = x4 * *global::<f32>(0x00E7CA0C);
        x4 *= x2;
        x0 += x1;
        x1 = x4 * *global::<f32>(0x00E7C9E0);
        x4 *= x2;
        x1 += x0;
        x0 = x4 * *global::<f32>(0x00E7C9E4);
        x4 *= x2;
        x0 += x1;
        x1 = x4 * *global::<f32>(0x00E7CA04);
        x4 *= x2;
        x1 += x0;
        x0 = x4 * *global::<f32>(0x00E7CA08);
        x4 *= x2;
        x0 += x1;
        x4 = x4 * *global::<f32>(0x00E7C9DC) + x0;
        if !(v - *global::<f32>(0x00E7C9F4) >= 0.0) {
            x4 = 0.0;
        }
        if v - *global::<f32>(0x00E7CA00) >= 0.0 {
            f32::from_bits(0xC2C8_0000)
        } else {
            x4
        }
    }
}
