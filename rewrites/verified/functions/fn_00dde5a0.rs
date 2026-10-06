// original: 0x00DDE5A0 UITextField key classifier
/// Return 1 when `key` is an accepted key, else 0: accepted keys are
/// 0xA1 | 0xAB | 0xBB | 0xBF | 0xC1 | 0xC9 | 0xCD | 0xD1 | 0xD3 | 0xDA | 0xDC | 0xE1 | 0xE9 | 0xED | 0xF1 | 0xF3 | 0xFA | 0xFC (anything else, including out-of-range values which take the
/// default, returns 0). The range check is UNSIGNED (ja).
/// Takes no object (ecx is ignored). Original: one stack word; the
/// Out-of-range keys take the default, which clears only al: the upper
/// 24 bits of the subtracted index are preserved.
lf_checker_rt::export!(thiscall, rw_00DDE5A0(_this: u32, key: u32) -> u32 {
    let idx = key.wrapping_sub(0xA1);
    if idx > 0x5B { idx & 0xFFFF_FF00 }
    else { (matches!(key, 0xA1 | 0xAB | 0xBB | 0xBF | 0xC1 | 0xC9 | 0xCD | 0xD1 | 0xD3 | 0xDA | 0xDC | 0xE1 | 0xE9 | 0xED | 0xF1 | 0xF3 | 0xFA | 0xFC)) as u32 }
});
