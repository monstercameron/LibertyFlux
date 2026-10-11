// original: 0x00A3D4D0 CVehicle::vf0

/// Run the base vehicle update first. If bit zero of the stack flag is set,
/// call the update helper with the current global vehicle manager as `this`
/// and this vehicle as its one stack argument. Return the receiver pointer.
/// The flag is tested as a bit mask; other bits do not affect the branch. The
/// original method is thiscall with one stack argument, which it returns from
/// the stack itself.
lf_checker_rt::export!(thiscall, rw_00a3d4d0(this: u32, flags: u32) -> u32 {
    const BASE_UPDATE: u32 = 1;
    const MANAGER_UPDATE: u32 = 2;
    const MANAGER_POINTER_VA: u32 = 0x012e_22a4;
    const UPDATE_FLAG: u32 = 1;

    let _ = lf_checker_rt::callee_thiscall!(BASE_UPDATE, u32, this);
    if flags & UPDATE_FLAG != 0 {
        let manager = unsafe { lf_checker_rt::global::<u32>(MANAGER_POINTER_VA).read_unaligned() };
        let _ = lf_checker_rt::callee_thiscall!(MANAGER_UPDATE, u32, manager, this);
    }
    this
});
