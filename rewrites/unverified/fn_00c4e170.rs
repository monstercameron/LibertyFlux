// original: 0x00C4E170 CTaskComplexAimAndThrowProjectile::~CTaskComplexAimAndThrowProjectile__deleting

/// Destroy the concrete task, then conditionally release its allocation.
///
/// `this` is passed in ECX and `delete_flag` is one stack word. The base
/// destructor always runs first and its EAX result is retained when bit 0
/// of `delete_flag` is clear. When that bit is set, the task allocator is
/// called with the manager pointer read from the global manager slot and
/// `this`; the function then returns `this`. The callee removes the one
/// stack word before this method returns.
lf_checker_rt::export!(thiscall, rw_00c4e170(this: u32, delete_flag: u32) -> u32 {
    const BASE_DTOR: u32 = 0;
    const RELEASE_TASK: u32 = 1;
    const MANAGER_VA: u32 = 0x167e_2a0;
    let base_result = unsafe {
        lf_checker_rt::callee_thiscall!(BASE_DTOR, u32, this)
    };
    if delete_flag & 1 == 0 {
        return base_result;
    }
    let manager = unsafe {
        lf_checker_rt::global::<u32>(MANAGER_VA).read_unaligned()
    };
    unsafe { lf_checker_rt::callee_thiscall!(RELEASE_TASK, u32, manager, this) };
    this
});
