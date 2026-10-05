// original: 0x00d56e30 ccam_photo_ctor

/// Camera-object constructor: run the shared base constructor, then install
/// this class's virtual table.
///
/// `this` points to the (uninitialised) object. The base constructor takes no
/// arguments and returns nothing observed; it runs first with `this` in ECX.
/// Word 0 of the object (the vtable slot) is then set to the class table
/// `VTABLE` (a file address; the original's store carries a relocation
/// entry, so the value written is the relocated address). Returns `this`.
///
/// Original: 0x00d56e30 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00d56e30(this: u32) -> u32 {
    unsafe {
        /// Class virtual table (file address; relocated at load).
        const VTABLE: u32 = 0x00ee65ac;
        /// Shared base constructor (intercepted; thiscall, no arguments).
        const BASE_CTOR: u32 = 1;
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        this
    }
});
