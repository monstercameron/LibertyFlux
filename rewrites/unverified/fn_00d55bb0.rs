// original: 0x00d55bb0 ccam_idle_ctor

/// Construct the idle camera: base, sub-object at `+0x140`, class init.
///
/// `this` points to the (uninitialised) object. The shared base constructor
/// runs first with `this` in ECX, word 0 (the vtable slot) is set to the
/// class table `VTABLE` (a file address; the original's store carries a
/// relocation entry), the sub-object constructor runs with `this + SUB` in
/// ECX, and the class initialiser runs with `this` in ECX. Returns `this`.
///
/// Original: 0x00d55bb0 (thiscall, no stack arguments, three calls).
lf_checker_rt::export!(thiscall, rw_00d55bb0(this: u32) -> u32 {
    unsafe {
        /// Class virtual table (file address; relocated at load).
        const VTABLE: u32 = 0x00ee6444;
        /// Sub-object offset within the object.
        const SUB: u32 = 0x140;
        /// Shared base constructor (intercepted; thiscall, no arguments).
        const BASE_CTOR: u32 = 1;
        /// Sub-object constructor (intercepted; thiscall, no arguments).
        const SUB_CTOR: u32 = 2;
        /// Class initialiser (intercepted; thiscall, no arguments).
        const CLASS_INIT: u32 = 3;
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        lf_checker_rt::callee_thiscall!(SUB_CTOR, u32, this.wrapping_add(SUB));
        lf_checker_rt::callee_thiscall!(CLASS_INIT, u32, this);
        this
    }
});
