// original: 0x00d547e0 ccam_debug_ctor

/// Camera-object constructor: run the shared base constructor, install this
/// class's virtual table, then run the class-specific initialiser.
///
/// `this` points to the (uninitialised) object. The base constructor runs
/// first with `this` in ECX and returns nothing observed. Word 0 of the
/// object (the vtable slot) is then set to the class table `VTABLE` (a file
/// address: the original's store carries a relocation entry), and the
/// class initialiser runs with `this` in ECX. Returns `this`.
///
/// Original: 0x00d547e0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00d547e0(this: u32) -> u32 {
    unsafe {
        /// Class virtual table (file address; relocated at load).
        const VTABLE: u32 = 0x00ee5db4;
        /// Shared base constructor (intercepted; thiscall, no arguments).
        const BASE_CTOR: u32 = 1;
        /// Class-specific initialiser (intercepted; thiscall, no arguments).
        const CLASS_INIT: u32 = 2;
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        lf_checker_rt::callee_thiscall!(CLASS_INIT, u32, this);
        this
    }
});
