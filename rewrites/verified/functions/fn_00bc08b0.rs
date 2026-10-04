// original: 0x00bc08b0 task_request_57 (proposed)

/// Request task kind 0x57: fold 7 condition words into a flag
/// word, then forward it with a float word and an extra word to the shared
/// dispatcher. Conditions: a4!=0 sets 0x10, a5!=0 sets 0x2, a6!=0 sets 0x1, a7!=0 sets 0x8, a8!=0 sets 0x20, a9!=0 sets 0x80, a10!=0 sets 0x40.
///
/// Forwards (TAG=0x57, a0, a1, a2, MODE=3, fbits, extra,
/// flags, 0) as nine words: the tag, the first three incoming words
/// unchanged, the mode, the float word, the extra word, the flags and
/// a zero. Returns the dispatcher's answer.
///
/// Original: 0x00BC08B0 (cdecl, 12 stack words).
lf_checker_rt::export!(cdecl, rw_00bc08b0(a0: u32, a1: u32, a2: u32, fbits: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, a9: u32, a10: u32, extra: u32) -> u32 {
    const TAG: u32 = 0x57;
    const MODE: u32 = 3;
    const BIT_A4: u32 = 0x10;
    const BIT_A5: u32 = 0x2;
    const BIT_A6: u32 = 0x1;
    const BIT_A7: u32 = 0x8;
    const BIT_A8: u32 = 0x20;
    const BIT_A9: u32 = 0x80;
    const BIT_A10: u32 = 0x40;
    let mut flags: u32 = 0;
    if a4 != 0 { flags |= BIT_A4; }
    if a5 != 0 { flags |= BIT_A5; }
    if a6 != 0 { flags |= BIT_A6; }
    if a7 != 0 { flags |= BIT_A7; }
    if a8 != 0 { flags |= BIT_A8; }
    if a9 != 0 { flags |= BIT_A9; }
    if a10 != 0 { flags |= BIT_A10; }
    lf_checker_rt::callee_cdecl!(1, u32, TAG, a0, a1, a2, MODE, fbits, extra, flags, 0)
});
