// original: 0x00DDE500 UITextField key classifier
/// Return 1 when `key` is an accepted key, else 0: accepted keys are
/// 0xC0 | 0xC1 | 0xC8 | 0xC9 | 0xCC | 0xCD | 0xD2 | 0xD3 | 0xD9 | 0xDA | 0xE0 | 0xE1 | 0xE8 | 0xE9 | 0xEC | 0xED | 0xF2 | 0xF3 | 0xF9 | 0xFA (anything else, including out-of-range values which take the
/// default, returns 0). The range check is UNSIGNED (ja).
/// Takes no object (ecx is ignored). Original: one stack word; the
/// Out-of-range keys take the default, which clears only al: the upper
/// 24 bits of the subtracted index are preserved.
lf_checker_rt::export!(thiscall, rw_00DDE500(_this: u32, key: u32) -> u32 {
    let idx = key.wrapping_sub(0xC0);
    if idx > 0x3A { idx & 0xFFFF_FF00 }
    else { (matches!(key, 0xC0 | 0xC1 | 0xC8 | 0xC9 | 0xCC | 0xCD | 0xD2 | 0xD3 | 0xD9 | 0xDA | 0xE0 | 0xE1 | 0xE8 | 0xE9 | 0xEC | 0xED | 0xF2 | 0xF3 | 0xF9 | 0xFA)) as u32 }
});
