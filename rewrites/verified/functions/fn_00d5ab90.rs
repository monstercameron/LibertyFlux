// original: 0x00d5ab90 ccam_sector_ctor
/// Constructor for CCamSector: runs the base constructor, installs this class's
/// virtual table, then runs the class-specific initialiser.
///
/// Calls the base constructor (intercepted callee 1, thiscall, no stack
/// arguments), writes the relocated address of the class table `VTABLE` at
/// `[this]`, then calls the initialiser (intercepted callee 2, thiscall, no
/// stack arguments) with the object pointer and returns it.
///
/// Original: thiscall, no stack arguments, returns `this`.
lf_checker_rt::export!(thiscall, rw_00d5ab90 (this: u32) -> u32 {
    unsafe {
        const BASE_CTOR: u32 = 1;
        const INIT: u32 = 2;
        const VTABLE: u32 = 0x00EE7F24;
        let _: u32 = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        let _: u32 = lf_checker_rt::callee_thiscall!(INIT, u32, this);
        this
    }
});
