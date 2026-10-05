// original: 0x00cacec0 CTaskComplexMoveGetToPointContinuous::~CTaskComplexMoveGetToPointContinuous
/// Destructor step: install two vtable pointers, then hand off to the next
/// destructor in the chain.
///
/// Takes the object pointer in ECX. Writes the primary vtable pointer at
/// the object's head and the secondary vtable pointer one word past the
/// embedded base, then transfers control to the chained destructor with
/// the same object pointer, returning whatever that call answers.
export!(thiscall, rw_00cacec0(this: u32) -> u32 {
    unsafe {
        *(this as *mut u32) = relocated(0x00ED8B0C);
        *(this.wrapping_add(0x14) as *mut u32) = relocated(0x00ED8B64);
        callee_thiscall!(1, u32, this)
    }
});
