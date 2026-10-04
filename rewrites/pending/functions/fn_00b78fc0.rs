// original: 0x00b78fc0 CTaskComplexMove::~CTaskComplexMove
/// Destroy a `CTaskComplexMove`, tail-calling the base destructor.
///
/// Restores the base-class virtual table pointers, then tail-calls the
/// `CTaskComplex` destructor with the same object pointer.
export!(thiscall, rw_00b78fc0(this: u32) -> u32 {
    unsafe {
        *(this as *mut u32) = relocated(0x00EB2D5C);
        *((this + 0x14) as *mut u32) = relocated(0x00EB2B98);
    }
    callee_thiscall!(1, u32, this)
});
