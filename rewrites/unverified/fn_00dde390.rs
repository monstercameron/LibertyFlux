// original: 0x00DDE390 UITextField key classifier
/// Return 1 when `key` is an accepted key, else 0: accepted keys are
/// 0x23..=0x29 | 0x2C..=0x2E | 0x30..=0x39 | 0x40..=0x5B | 0x5D..=0x7B | 0x7D (anything else, including out-of-range values which take the
/// default, returns 1). The range check is UNSIGNED (ja).
/// Takes no object (ecx is ignored). Original: one stack word; the
/// Out-of-range keys take the default, which sets only al: the upper 24
/// bits of the subtracted index are preserved.
lf_checker_rt::export!(thiscall, rw_00DDE390(_this: u32, key: u32) -> u32 {
    let idx = key.wrapping_sub(0x22);
    if idx > 0x5C { (idx & 0xFFFF_FF00) | 1 }
    else { (matches!(key, 0x23..=0x29 | 0x2C..=0x2E | 0x30..=0x39 | 0x40..=0x5B | 0x5D..=0x7B | 0x7D)) as u32 }
});
