// original: 0x00be7a20 CTaskSimpleWaitUntilAreaCodesMatch::vf17

/// Wait until area codes match, delegating to the pathfind-problem task.
///
/// `this` points to the task, `ped` to the ped. Returns 1 at once when the
/// area handle at `+0x50` is null (nothing to wait for). Returns 0 while bit
/// 2 of the ped flags at `+0x26c` is set. Otherwise tail-jumps to the sibling
/// `CTaskSimplePathfindProblem::vf17` entry with the same arguments; the
/// rewrite forwards that tail call through the checker's intercepted callee 1
/// (thiscall, one word: the ped) and returns its answer. (The original
/// rewrites its incoming argument slot with the same ped value before the
/// jump, so the final stack diff is empty.)
///
/// Original: thiscall, one stack word, the callee pops 4 bytes, returns `al`.
lf_checker_rt::export!(thiscall, rw_00be7a20(this: u32, ped: u32) -> u32 {
    unsafe {
        const OFF_AREA: u32 = 0x50;
        const PED_FLAGS: u32 = 0x26c;
        const WAIT_BIT: u8 = 4;
        const SIBLING: u32 = 1;

        if ((this + OFF_AREA) as *const u32).read_unaligned() == 0 {
            return 1;
        }
        if ((ped + PED_FLAGS) as *const u8).read() & WAIT_BIT != 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(SIBLING, u32, this, ped)
    }
});
