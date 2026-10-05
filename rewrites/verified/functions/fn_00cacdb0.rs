// original: 0x00cacdb0 CTaskComplexGoToPointAnyMeans::~CTaskComplexGoToPointAnyMeans
/// Destructor step: install the vtable, release the embedded member, hand off.
///
/// Takes the object pointer in ECX. Writes the vtable pointer(s) at the
/// object's head, then, when the member's tag word is non-null, calls the
/// member release helper with a pointer to the member itself. Transfers
/// control to the chained destructor with the same object pointer,
/// returning whatever that call answers.
export!(thiscall, rw_00cacdb0(this: u32) -> u32 {
    unsafe {
        *(this as *mut u32) = relocated(0x00ED88C4);
        let tag = *(this.wrapping_add(0x3C) as *const u32);
        if tag != 0 {
            callee_stdcall!(2, u32, this.wrapping_add(0x3C));
        }
        callee_thiscall!(1, u32, this)
    }
});
