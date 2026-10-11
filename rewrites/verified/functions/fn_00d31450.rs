// original: 0x00D31450 CTaskComplexNewExitVehicle::~CTaskComplexNewExitVehicle__deleting
/// Runs the class destructor, then invokes the deleting helper only when bit
/// zero of `flags` is set. The allocator pointer is read from the shared task
/// pool slot; the object pointer is passed as the helper's sole stack argument.
/// The object pointer is returned. The call order is base destructor, then
/// optional delete helper.
lf_checker_rt::export!(thiscall, rw_00d31450(this: u32, flags: u32) -> u32 {
    const TASK_POOL_GLOBAL_FILE_VA: u32 = 0x0167_e2a0;
    const SHOULD_DELETE: u32 = 1;

    let flags = flags & 0xff;
    let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
    if flags & SHOULD_DELETE != 0 {
        let allocator = unsafe { lf_checker_rt::global::<u32>(TASK_POOL_GLOBAL_FILE_VA).read_unaligned() };
        let _ = lf_checker_rt::callee_thiscall!(2, u32, allocator, this);
    }
    this
});
