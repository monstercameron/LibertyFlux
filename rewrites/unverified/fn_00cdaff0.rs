// original: 0x00CDAFF0 CTaskSimpleNMFlinch::vf0
/// Run the base destructor for this task and, when bit 0 of `flags` is set, notify the script context loaded from the global slot so it releases the task. The base destructor result and the notify result are both ignored. The function returns the original task pointer. Calling convention: thiscall with one stack word; only bit 0 of the flags word is tested.
lf_checker_rt::export!(thiscall, rw_00cdaff0(this: u32, flags: u32) -> u32 {
    const SCRIPT_CONTEXT_POINTER_VA: u32 = 0x0167_E2A0;
    const DELETE_FLAG: u32 = 0x01;
    const BASE_DESTRUCTOR: u32 = 1;
    const NOTIFY_SCRIPT_CONTEXT: u32 = 2;
    let _ = lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR, u32, this);
    if flags & DELETE_FLAG != 0 {
        let ctx = unsafe { lf_checker_rt::global::<u32>(SCRIPT_CONTEXT_POINTER_VA).read_unaligned() };
        let _ = lf_checker_rt::callee_thiscall!(NOTIFY_SCRIPT_CONTEXT, u32, ctx, this);
    }
    this
});
