// original: 0x00b1cf70 override_or_default (proposed)

/// Returns a global override value, or the default argument.
///
/// Cdecl of two stack words: a default value and a flag byte. When the
/// flag is nonzero and the override-enabled byte is clear, returns the
/// override dword; otherwise returns the default. Only the flag's low
/// byte is read.
lf_checker_rt::export!(cdecl, rw_00b1cf70(default: u32, flag: u32) -> u32 {
    unsafe {
        const ENABLED: u32 = 0x016337f8;
        const OVERRIDE: u32 = 0x016337fc;
        if (flag & 0xff) != 0 && (lf_checker_rt::global::<u8>(ENABLED) as *const u8).read() == 0 {
            (lf_checker_rt::global::<u32>(OVERRIDE) as *const u32).read_unaligned()
        } else {
            default
        }
    }
});
