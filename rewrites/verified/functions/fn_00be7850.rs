// original: 0x00be7850 CTaskSimpleStandUp::vf17

/// Report whether a stand-up task has finished, starting it on first call.
///
/// `this` points to the task, `ped` to the ped. If the done flag at `+0x20`
/// is set, returns 1 at once. Otherwise, if the started field at `+0x14` is
/// still zero, starts the underlying action through callee 1 (thiscall with
/// `this` in `ecx`, one stack word: the ped) and returns 0 either way.
///
/// Original: thiscall, one stack word, the callee pops 4 bytes, returns `al`.
lf_checker_rt::export!(thiscall, rw_00be7850(this: u32, ped: u32) -> u32 {
    unsafe {
        const OFF_STARTED: u32 = 0x14;
        const OFF_DONE: u32 = 0x20;
        const STARTER: u32 = 1;

        if ((this + OFF_DONE) as *const u8).read() != 0 {
            return 1;
        }
        if ((this + OFF_STARTED) as *const u32).read_unaligned() == 0 {
            lf_checker_rt::callee_thiscall!(STARTER, u32, this, ped);
        }
        0
    }
});
