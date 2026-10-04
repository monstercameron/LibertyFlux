// original: 0x00b78e60 CTaskComplex::CTaskComplex
/// Construct a `CTaskComplex` in place.
///
/// Runs the base `CTask` constructor on the object, then installs this
/// class's virtual table pointer. Returns the object pointer.
export!(thiscall, rw_00b78e60(this: u32) -> u32 {
    let _: u32 = callee_thiscall!(1, u32, this);
    unsafe {
        *(this as *mut u32) = relocated(0x00EB2C74);
    }
    this
});
