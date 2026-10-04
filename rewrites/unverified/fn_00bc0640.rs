// original: 0x00bc0640 task_request_59_const (proposed)

/// Request task kind 0x59: forward fixed words, the first three
/// incoming words and one float word to the shared dispatcher. Forwards
/// (TAG=0x59, a0, a1, a2, MODE=3, fbits, -1, 0x90, 0) as nine words.
/// Returns the dispatcher's answer.
///
/// Original: 0x00BC0640 (cdecl, 4 stack words).
lf_checker_rt::export!(cdecl, rw_00bc0640(a0: u32, a1: u32, a2: u32, fbits: u32) -> u32 {
    const TAG: u32 = 0x59;
    const FLAGS: u32 = 0x90;
    const MODE: u32 = 3;
    lf_checker_rt::callee_cdecl!(1, u32, TAG, a0, a1, a2, MODE, fbits, 0xFFFF_FFFF, FLAGS, 0)
});
