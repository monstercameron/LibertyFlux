// original: 0x00CD7770 CTaskComplexFollowLeaderInFormation::~CTaskComplexFollowLeaderInFormation__deleting

/// Destroy the class-specific part of this task and optionally release the
/// object to its pool. `this` is the task object and `flags` is the scalar
/// deleting-destructor flag word; only bit zero of its low byte selects the
/// pool-release call. The pool context is read from its global slot, and the
/// object pointer is returned unchanged.
///
/// Calling convention: thiscall with one 32-bit stack argument. The class
/// destructor runs first. The pool helper runs only when the delete flag is
/// set; both calls are forwarded through the contract's scripted stubs.
lf_checker_rt::export!(thiscall, rw_00cd7770(this: u32, flags: u32) -> u32 {
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
