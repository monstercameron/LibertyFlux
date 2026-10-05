// original: 0x00884d20 stream_mode_classify (proposed)
/// Classify a (mode, submode) pair into a small outcome code.
///
/// The original dispatches through a jump table; the rewrite states the same
/// mapping directly. Mode 1 accepts submodes 2, 3, 5 and 6 (returning 1)
/// and rejects the rest (returning 0); mode 2 always returns 2; modes 3, 5
/// and 6 always return 1; every other mode (0, 4, 7, 8 and anything above)
/// returns 0.
///
/// Original: cdecl, two stack arguments, returns in `eax`.
lf_checker_rt::export!(cdecl, rw_00884d20(mode: u32, sub: u32) -> u32 {
    match mode {
        1 => {
            if sub == 2 || sub == 3 || sub == 5 || sub == 6 {
                1
            } else {
                0
            }
        }
        2 => 2,
        3 | 5 | 6 => 1,
        _ => 0,
    }
});
