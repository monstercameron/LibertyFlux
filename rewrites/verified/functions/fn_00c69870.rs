// original: 0x00c69870 stream_rebuild_if_dirty (proposed)

/// Rebuild the streaming state unless it is already clean.
///
/// The dirtiness check runs first; a nonzero low byte returns its full
/// answer at once. Otherwise three rebuild passes run on `this` and
/// control transfers to the finaliser (a tail call: the rewrite calls
/// it and returns its answer).
///
/// Original: thiscall with no stack words, four calls plus a tail call.
lf_checker_rt::export!(thiscall, rw_00c69870(this: u32) -> u32 {
    unsafe {
        const DIRTY: u32 = 1;
        const PASS_A: u32 = 2;
        const PASS_B: u32 = 3;
        const PASS_C: u32 = 4;
        const FINISH: u32 = 5;

        let d: u32 = lf_checker_rt::callee_thiscall!(DIRTY, u32, this);
        if (d & 0xFF) != 0 {
            return d;
        }
        lf_checker_rt::callee_thiscall!(PASS_A, u32, this);
        lf_checker_rt::callee_thiscall!(PASS_B, u32, this);
        lf_checker_rt::callee_thiscall!(PASS_C, u32, this);
        lf_checker_rt::callee_thiscall!(FINISH, u32, this)
    }
});
