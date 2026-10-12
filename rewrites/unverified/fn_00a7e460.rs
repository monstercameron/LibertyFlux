// original: 0x00A7E460 CComplexGangDrivebyTaskInfo::~CComplexGangDrivebyTaskInfo__deleting
/// Run the CComplexGangDrivebyTaskInfo base destructor. If bit 0 of `delete_flags` is set, release this object through the manager pointer at the global manager slot. Return `this` and clean the one stack flag word.
lf_checker_rt::export!(thiscall, rw_00a7e460(this: u32, delete_flags: u32) -> u32 {
    const BASE_DESTRUCTOR: u32 = 15;
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
