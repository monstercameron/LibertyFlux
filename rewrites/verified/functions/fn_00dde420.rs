// original: 0x00DDE420 UITextField key classifier
/// Return 1 when `key` is an accepted key, else 0: accepted keys are
/// 0xC0 | 0xC2 | 0xC6..=0xCB | 0xCE | 0xCF | 0xD4 | 0xD9 | 0xDB | 0xDC | 0xE0 | 0xE2 | 0xE6..=0xEB | 0xEE | 0xEF | 0xF4 | 0xF9 | 0xFB | 0xFC (anything else, including out-of-range values which take the
/// default, returns 0). The range check is UNSIGNED (ja).
/// Takes no object (ecx is ignored). Original: one stack word; the
/// Out-of-range keys take the default, which clears only al: the upper
/// 24 bits of the subtracted index are preserved.
lf_checker_rt::export!(thiscall, rw_00DDE420(_this: u32, key: u32) -> u32 {
    let idx = key.wrapping_sub(0xC0);
    if idx > 0x3C { idx & 0xFFFF_FF00 }
    else { (matches!(key, 0xC0 | 0xC2 | 0xC6..=0xCB | 0xCE | 0xCF | 0xD4 | 0xD9 | 0xDB | 0xDC | 0xE0 | 0xE2 | 0xE6..=0xEB | 0xEE | 0xEF | 0xF4 | 0xF9 | 0xFB | 0xFC)) as u32 }
});
