// original: 0x00d587a0 ccam_weapon_aiming_ctor
/// Constructor fragment for CCamWeaponAiming: runs the base constructor, then installs
/// this class's virtual table.
///
/// Calls the base constructor (intercepted callee 1, thiscall, no stack
/// arguments) with the object pointer, writes the relocated address of the
/// class table `VTABLE` at `[this]`, and returns the object pointer.
///
/// Original: thiscall, no stack arguments, returns `this`.
lf_checker_rt::export!(thiscall, rw_00d587a0 (this: u32) -> u32 {
    unsafe {
        const BASE_CTOR: u32 = 1;
        const VTABLE: u32 = 0x00EE7550;
        let _: u32 = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        this
    }
});
