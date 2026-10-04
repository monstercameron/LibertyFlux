// original: 0x00bc0930 task_request_58_mode (proposed)

/// Request task kind 0x58: pick a mode from two bits of a selector word,
/// then forward it with the selector and a float word to the shared
/// dispatcher. Mode is 6 when bit 9 is set, else 4 when bit 11 is set,
/// else 3. Forwards (TAG=0x58, a0, a1, a2, mode, fbits, extra,
/// modebits, 0) as nine words. Returns the shifted selector when bit 9
/// is set, else 4.
///
/// Original: 0x00BC0930 (cdecl, 6 stack words).
lf_checker_rt::export!(cdecl, rw_00bc0930(a0: u32, a1: u32, a2: u32, fbits: u32, extra: u32, modebits: u32) -> u32 {
    const TAG: u32 = 0x58;
    const BIT_HI: u32 = 1 << 9;
    const BIT_LO: u32 = 1 << 11;
    let mode: u32 = if modebits & BIT_HI != 0 { 6 } else if modebits & BIT_LO != 0 { 4 } else { 3 };
    let retv: u32 = if modebits & BIT_HI != 0 { modebits >> 9 } else { 4 };
    lf_checker_rt::callee_cdecl!(1, u32, TAG, a0, a1, a2, mode, fbits, extra, modebits, 0);
    retv
});
