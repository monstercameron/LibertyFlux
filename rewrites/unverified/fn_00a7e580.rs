// original: 0x00A7E580 CMeleeTaskInfo::~CMeleeTaskInfo__deleting
/// Install the CMeleeTaskInfo vtable, run its CTargetTaskInfo base destructor, and release this object through the global manager when bit 0 of `delete_flags` is set. Return `this` and clean the flag word.
lf_checker_rt::export!(thiscall, rw_00a7e580(this: u32, delete_flags: u32) -> u32 {
    const BASE_DESTRUCTOR: u32 = 19;
    const RELEASE_SLOT: u32 = 17;
    const MANAGER_SLOT: u32 = 0x012FB1AC;
    const VTABLE: u32 = 0x00EA123C;
    const DELETE_FLAG: u32 = 1;
    unsafe { (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE)); }
    let _ = lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR, u32, this);
    if delete_flags & DELETE_FLAG != 0 {
        let manager = unsafe { lf_checker_rt::global::<u32>(MANAGER_SLOT).read_unaligned() };
        let _ = lf_checker_rt::callee_thiscall!(RELEASE_SLOT, u32, manager, this);
    }
    this
});
