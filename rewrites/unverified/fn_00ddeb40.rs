// original: 0x00DDEB40 UITextField conditional update
/// UITextField helper (input-ui subsystem). `this` is the field object.
/// Refresh the field and report success. Read the grandchild field, run the
/// updater with (`key`, 0) and validate: when `force` (low byte) is nonzero,
/// return the validation result as is; when it is zero and validation passed
/// (low byte nonzero), return 1 BUT with the validation's upper 24 bits kept
/// (only al is written); when validation failed, re-run the updater with the
/// saved field value, validate once more and return the second validation's upper 24 bits
/// with a zero low byte (only al is cleared).
/// Original: thiscall, two stack words, result in eax.
lf_checker_rt::export!(thiscall, rw_00DDEB40(this: u32, key: u32, force: u32) -> u32 {
    unsafe {
        const READ_FIELD: u32 = 1;
        const UPDATE: u32 = 2;
        const VALIDATE: u32 = 3;
        let saved = lf_checker_rt::callee_thiscall!(READ_FIELD, u32, this);
        lf_checker_rt::callee_thiscall!(UPDATE, u32, this, key, 0);
        let ans = lf_checker_rt::callee_thiscall!(VALIDATE, u32, this);
        if (force as u8) != 0 {
            ans
        } else if (ans as u8) != 0 {
            (ans & 0xFFFF_FF00) | 1
        } else {
            lf_checker_rt::callee_thiscall!(UPDATE, u32, this, saved, 0);
            let ans2 = lf_checker_rt::callee_thiscall!(VALIDATE, u32, this);
            ans2 & 0xFFFF_FF00
        }
    }
});
