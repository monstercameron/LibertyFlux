// original: 0x00c08bd0 stream_record_activate_and_commit (proposed)

/// Activate the argument's record, then commit it.
///
/// `this` and `arg` go to the activator (callee 1, answer ignored) and then
/// to the committer (callee 2). Returns the committer's answer with its low
/// byte forced to 1.
///
/// Original: 0x00c08bd0 (thiscall, one stack word; both callees thiscall).
lf_checker_rt::export!(thiscall, rw_00c08bd0(this: u32, arg: u32) -> u32 {
    unsafe {
        const ACTIVATE: u32 = 1;
        const COMMIT: u32 = 2;
        let _s: u32 = lf_checker_rt::callee_thiscall!(ACTIVATE, u32, this, arg);
        let r: u32 = lf_checker_rt::callee_thiscall!(COMMIT, u32, this, arg);
        (r & 0xffff_ff00) | 1
    }
});
