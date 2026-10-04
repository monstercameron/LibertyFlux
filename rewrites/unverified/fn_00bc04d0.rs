// original: 0x00bc04d0 task_request_52 (proposed)

/// Request task kind 0x52: fold 2 condition words into a flag
/// word, then forward it with a float word and an extra word to the shared
/// dispatcher. Conditions: a4!=0 sets 0x10, a5!=0 sets 0x20.
/// A base mask of 0x680 is always set.
///
/// Forwards (TAG=0x52, a0, a1, a2, MODE=6, fbits, extra,
/// flags, 0) as nine words: the tag, the first three incoming words
/// unchanged, the mode, the float word, the extra word, the flags and
/// a zero. Returns the flags word.
///
/// Original: 0x00BC04D0 (cdecl, 7 stack words).
lf_checker_rt::export!(cdecl, rw_00bc04d0(a0: u32, a1: u32, a2: u32, fbits: u32, a4: u32, a5: u32, extra: u32) -> u32 {
    const TAG: u32 = 0x52;
    const MODE: u32 = 6;
    const BASE_FLAGS: u32 = 0x680;
    const BIT_A4: u32 = 0x10;
    const BIT_A5: u32 = 0x20;
    let mut flags: u32 = 0;
    if a4 != 0 { flags |= BIT_A4; }
    if a5 != 0 { flags |= BIT_A5; }
    flags |= BASE_FLAGS;
    lf_checker_rt::callee_cdecl!(1, u32, TAG, a0, a1, a2, MODE, fbits, extra, flags, 0)
});
