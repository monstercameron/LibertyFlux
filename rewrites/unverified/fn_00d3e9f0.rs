// original: 0x00D3E9F0 CTaskSimpleJumpLaunch::~CTaskSimpleJumpLaunch__deleting
/// Calls the class destructor helper, then invokes the global deletion helper
/// only when bit 0 of `delete_flag` is set. The helper receives the global
/// manager pointer in ECX and this object as its one stack argument. The
/// deleting destructor returns the object pointer.
lf_checker_rt::export!(thiscall, rw_d3e9f0(task: u32, delete_flag: u32) -> u32 {
    const CLASS_DESTRUCTOR: u32 = 1;
    const GLOBAL_DELETE_HELPER: u32 = 2;
    const DELETE_MANAGER_SLOT: u32 = 0x0167E2A0;
    let _ = lf_checker_rt::callee_thiscall!(CLASS_DESTRUCTOR, u32, task);
    if delete_flag & 1 != 0 {
        let manager = unsafe { lf_checker_rt::global::<u32>(DELETE_MANAGER_SLOT).read_unaligned() };
        let _ = lf_checker_rt::callee_thiscall!(GLOBAL_DELETE_HELPER, u32, manager, task);
    }
    task
});
