// original: 0x00DDEAE0 UITextField conditional scale update
/// UITextField helper (input-ui subsystem). `this` is the field object.
/// Refresh the field scale and report success. Measure the field, run the
/// scaler with (`scale`, 0) and validate: when `force` (low byte) is nonzero,
/// return the validation result as is; when it is zero and validation passed
/// (low byte nonzero), return 1 BUT with the validation's upper 24 bits kept
/// (only al is written); when validation failed, re-run the scaler with the
/// saved measurement, validate once more and return the second validation's
/// upper 24 bits with a zero low byte (only al is cleared).
/// Original: thiscall, two stack words (`scale` is float bits), result in eax.
lf_checker_rt::export!(thiscall, rw_00DDEAE0(this: u32, scale: u32, force: u32) -> u32 {
    unsafe {
        const MEASURE: u32 = 1;
        const SCALE: u32 = 2;
        const VALIDATE: u32 = 3;
        let m: f32 = lf_checker_rt::callee_thiscall!(MEASURE, f32, this);
        lf_checker_rt::callee_thiscall!(SCALE, u32, this, scale, 0);
        let ans = lf_checker_rt::callee_thiscall!(VALIDATE, u32, this);
        if (force as u8) != 0 {
            ans
        } else if (ans as u8) != 0 {
            (ans & 0xFFFF_FF00) | 1
        } else {
            lf_checker_rt::callee_thiscall!(SCALE, u32, this, m.to_bits(), 0);
            let ans2 = lf_checker_rt::callee_thiscall!(VALIDATE, u32, this);
            ans2 & 0xFFFF_FF00
        }
    }
});
