// original: 0x0086f450 CEIL
/// Script native `CEIL` (hash 0x76181322).
///
/// Reads one float script argument, widens it to a double, and calls the
/// engine's ceiling routine with that double. The engine returns the
/// ceiling in ST0; the handler converts it back to float, truncates toward
/// zero, and stores the integer into the return slot.
///
/// The stubbed callee also copies the scripted float bits to EAX, which is
/// what this rewrite converts. The conversion matches x86 `cvttss2si`:
/// out-of-range and NaN inputs yield `0x80000000`, unlike Rust's
/// saturating `as` cast.
export!(cdecl, rw_0086f450(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let wide = f32::from_bits(*args) as f64;
        let double_bits = wide.to_bits();
        let answer = callee_cdecl!(
            1,
            u32,
            double_bits as u32,
            (double_bits >> 32) as u32,
        );
        let result = f32::from_bits(answer);
        let truncated = if result.is_nan() || result >= 2147483648.0 || result < -2147483648.0 {
            0x80000000u32
        } else {
            result as i32 as u32
        };
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = truncated;
        slot as u32
    }
});
