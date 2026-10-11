// original: 0x00A62720 adjust_pinned_down_counter

/// Adjust the pinned-down counter at `this + 0x2c8` toward the requested
/// amount. The original converts its one `f32` stack argument with x86
/// `CVTTSS2SI`, then sign-extends the low byte of that integer. It subtracts
/// that signed byte from the current signed counter, stores the positive
/// difference clamped to 100 (or zero when non-positive), and returns the
/// signed input byte when the difference reaches the cap, otherwise the
/// stored difference. `this` is in ECX; `amount_bits` is one stack word and
/// the callee removes it.
lf_checker_rt::export!(thiscall, rw_00a62720(this: u32, amount_bits: u32) -> u32 {
    unsafe {
        const COUNTER: u32 = 0x2c8;
        const LIMIT: i32 = 100;
        let requested = f32::from_bits(amount_bits);
        let converted = if !requested.is_finite() || requested >= 2_147_483_648.0 || requested < -2_147_483_648.0 {
            i32::MIN
        } else {
            requested.trunc() as i32
        };
        let requested_byte = (converted as u8) as i8;
        let current = (this.wrapping_add(COUNTER) as *const u8).read() as i8;
        let difference = i32::from(current) - i32::from(requested_byte);
        let stored = if difference >= LIMIT { LIMIT as u8 } else if difference > 0 { difference as u8 } else { 0 };
        (this.wrapping_add(COUNTER) as *mut u8).write(stored);
        if difference >= LIMIT { i32::from(requested_byte) as u32 } else { difference.max(0) as u32 }
    }
});
