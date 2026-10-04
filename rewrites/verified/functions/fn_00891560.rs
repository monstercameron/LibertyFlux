// original: 0x00891560 aud_scaled_param_apply
/// Scales the float argument and applies it through the looked-up target.
///
/// Multiplies the argument by two constant factors, truncates toward zero,
/// resolves the target like `aud_param_apply_flagged`, and invokes the callee
/// with the target and the scaled value. Returns the callee's answer. A
/// leading no-argument call (whose answer is discarded) runs first.
///
/// Truncation uses `cvttss2si` below, which reproduces the x86 instruction:
/// NaN, infinities and out-of-range values yield `i32::MIN` instead of
/// saturating the way Rust's `as` cast does.
fn cvttss2si(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        i32::MIN
    } else {
        x as i32
    }
}
export!(thiscall, rw_00891560(this: *mut u8, arg_bits: u32) -> u32 {
    unsafe {
        let _: u32 = callee_thiscall!(1, u32, this as u32);
        let c1 = *global::<f32>(0xe78588);
        let c2 = *global::<f32>(0xe7858c);
        let scaled = cvttss2si(f32::from_bits(arg_bits) * c1 * c2);
        let b = *(this.add(4));
        let target = if b == 0xff {
            0
        } else {
            let stride = *global::<u32>(0x115d968);
            let table = *global::<u32>(0x115d988);
            let idx = *(this.add(0x40)) as u32;
            let entry = *((table
                .wrapping_add(idx.wrapping_mul(0x6f40))
                .wrapping_add(0x6f14)) as *const u32);
            stride.wrapping_mul(b as u32).wrapping_add(entry)
        };
        callee_thiscall!(2, u32, target, scaled as u32)
    }
});
