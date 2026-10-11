// original: 0x00CD7740 CTaskComplexFollowLeaderAnyMeans::~CTaskComplexFollowLeaderAnyMeans__deleting

/// Destroy the class-specific part of a task and optionally return the task
/// object to its pool. `this` is the object pointer and `flags` is the
/// deleting-destructor flag word; only bit zero controls the pool call. The
/// pool context is loaded from the game's global slot, passed with `this` to
/// the helper, and the function returns the original object pointer.
///
/// Calling convention: thiscall with one 32-bit stack argument. The base
/// destructor is called first; the pool helper follows only when bit zero is
/// set, matching the shared scalar deleting-destructor protocol.
lf_checker_rt::export!(thiscall, rw_00cd7740(this: u32, flags: u32) -> u32 {
    const POOL_CONTEXT_GLOBAL: u32 = 0x0167_E2A0;
    const DELETE_TO_POOL: u32 = 1;
    const BASE_DESTRUCTOR: u32 = 1;
    const POOL_FREE: u32 = 2;

    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR, u32, this);
        if flags & DELETE_TO_POOL != 0 {
            let pool_context = lf_checker_rt::global::<u32>(POOL_CONTEXT_GLOBAL).read();
            let _ = lf_checker_rt::callee_thiscall!(POOL_FREE, u32, pool_context, this);
        }
    }
    this
});
