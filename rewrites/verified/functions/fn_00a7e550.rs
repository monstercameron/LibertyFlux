// original: 0x00A7E550 CMedicTaskInfo::~CMedicTaskInfo__deleting
/// Run the CMedicTaskInfo base destructor. If bit 0 of `delete_flags` is set, release this object through the global manager. Return `this` and clean the flag word.
lf_checker_rt::export!(thiscall, rw_00a7e550(this: u32, delete_flags: u32) -> u32 {
    const BASE_DESTRUCTOR: u32 = 18;
    const RELEASE_SLOT: u32 = 17;
    const MANAGER_SLOT: u32 = 0x012FB1AC;
    const DELETE_FLAG: u32 = 1;
    let _ = lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR, u32, this);
    if delete_flags & DELETE_FLAG != 0 {
        let manager = unsafe { lf_checker_rt::global::<u32>(MANAGER_SLOT).read_unaligned() };
        let _ = lf_checker_rt::callee_thiscall!(RELEASE_SLOT, u32, manager, this);
    }
    this
});
