// original: 0x00b4f1e0 forward_status_arg (proposed)

/// Forward an argument to the current task's status hook.
///
/// The current task is fetched twice (callees 1 and 2, thiscall on `this`
/// with no arguments; a null first fetch returns 0 without calling again).
/// The second fetch's virtual slot at `+0x20` then runs (thiscall on the
/// task, passed `this` and `arg`) and its result is returned.
///
/// Original: 0x00b4f1e0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00b4f1e0(this: u32, arg: u32) -> u32 {
    unsafe {
        const FETCH: u32 = 1;
        const FETCH_AGAIN: u32 = 2;
        const STATUS_SLOT: u32 = 0x20;
        let first = lf_checker_rt::callee_thiscall!(FETCH, u32, this);
        if first == 0 {
            return 0;
        }
        let task = lf_checker_rt::callee_thiscall!(FETCH_AGAIN, u32, this);
        let vt = (task as *const u32).read_unaligned();
        let status: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(((vt + STATUS_SLOT) as *const u32).read_unaligned() as usize);
        status(task, this, arg)
    }
});
