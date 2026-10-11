// original: 0x00c453c0 CCamFinal::vf0
/// Destroy the final camera, optionally detach it from the global camera
/// manager when delete-flag bit zero is set, and return this object pointer.
/// The manager pointer is read from its relocated global slot; removal is a
/// thiscall with this camera as its one stack argument.
lf_checker_rt::export!(thiscall, rw_00c453c0(this: u32, delete_flags: u32) -> u32 {
    const DESTROY_CAMERA: u32 = 1;
    const REMOVE_FROM_MANAGER: u32 = 2;
    const MANAGER_SLOT_VA: u32 = 0x012f_b1a0;
    lf_checker_rt::callee_thiscall!(DESTROY_CAMERA, u32, this);
    if delete_flags & 1 != 0 {
        let manager = unsafe {
            lf_checker_rt::global::<u32>(MANAGER_SLOT_VA).read_unaligned()
        };
        lf_checker_rt::callee_thiscall!(REMOVE_FROM_MANAGER, u32, manager, this);
    }
    this
});
