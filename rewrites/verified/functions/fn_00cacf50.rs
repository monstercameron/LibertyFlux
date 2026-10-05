// original: 0x00cacf50 CTaskComplexMoveGoToShelterAndWait::~CTaskComplexMoveGoToShelterAndWait
/// Destructor step: install the vtables, release the owned field, hand off.
///
/// Takes the object pointer in ECX. Writes the two vtable pointers at the
/// object's head, then, when the owned field is non-null, calls the
/// release helper with the field value, leaving the field itself in place.
/// Transfers control to the chained destructor with the same object
/// pointer, returning whatever that call answers.
export!(thiscall, rw_00cacf50(this: u32) -> u32 {
    unsafe {
        *(this as *mut u32) = relocated(0x00ED8D4C);
        *(this.wrapping_add(0x14) as *mut u32) = relocated(0x00ED8DA4);
        let field = *(this.wrapping_add(0x30) as *const u32);
        if field != 0 {
            callee_cdecl!(2, u32, field);
        }
        callee_thiscall!(1, u32, this)
    }
});
