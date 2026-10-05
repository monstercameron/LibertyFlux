// original: 0x00a097f0 mission_cleanup_kind_allowed (proposed)
/// Test whether a cleanup record kind is processed in this mode.
///
/// Asks the cleanup gate first; when it says no the answer is 0. Otherwise
/// kinds 1, 2, 4 and 7 are processed and every other kind is not, matching
/// the original's two-level jump table exactly. Only the low byte is set.
/// Stdcall, one word.
lf_checker_rt::export!(stdcall, rw_00a097f0(kind: u32) -> u32 {
    unsafe {
        const GATE: u32 = 0;
        let armed: u32 = lf_checker_rt::callee_cdecl!(GATE, u32,);
        if (armed & 0xff) == 0 {
            return 0;
        }
        match kind & 0xff {
            1 | 2 | 4 | 7 => 1,
            _ => 0,
        }
    }
});
