// original: 0x00A967C0 emit_timer_steps_10
/// Drive the sink once per loop step at a coarser scale (original 0x00A967C0).
///
/// Sibling of the 100-scale driver: samples the same timer, scales it by ten
/// and splits it into a whole step count and a fraction. Each step hands the
/// sink an eight-word frame holding the ramp before and after adding the
/// tenth-step among constant zero, zero, one and one words; a nonzero
/// fraction adds one closing frame carrying the fraction and the final ramp.
/// The sink's third argument is the timer call's register residue (zero under
/// the checker), and the function returns the trailing check's answer.
export!(thiscall, rw_a967c0(_this: u32) -> u32 {
    const SCALE: f32 = 10.0;
    const STEP: f32 = f32::from_bits(0x3DCC_CCCD); // 0.1
    const ONE: u32 = 0x3F80_0000;
    let sample: f64 = callee_cdecl!(1, f64,);
    let scaled = (sample as f32) * SCALE;
    let steps = cvttss2si(scaled);
    let frac = scaled - steps as f32;
    let mut ramp = 0f32;
    if steps > 0 {
        for _ in 0..steps {
            let before = ramp;
            ramp += STEP;
            let frame = [
                before.to_bits(),
                0u32,
                ramp.to_bits(),
                0,
                ramp.to_bits(),
                ONE,
                before.to_bits(),
                ONE,
            ];
            callee_cdecl!(2, u32, frame.as_ptr() as u32, 0, 0);
        }
    }
    if frac != 0.0 {
        let after = ramp + STEP;
        let frame = [
            ramp.to_bits(),
            0u32,
            after.to_bits(),
            0,
            after.to_bits(),
            frac.to_bits(),
            ramp.to_bits(),
            frac.to_bits(),
        ];
        callee_cdecl!(2, u32, frame.as_ptr() as u32, 0, 0);
    }
    callee_cdecl!(3, u32,) });

/// Truncating f32-to-i32 conversion with x86 `cvttss2si` semantics: NaN and
/// out-of-range inputs (including infinities) yield `i32::MIN`, everything
/// else truncates toward zero. Rust's `as` cast saturates instead, so the
/// guard matters at the edges. (Shared helper, also used by fn_00A96610.)
fn cvttss2si(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        i32::MIN
    } else {
        x as i32
    }
}
