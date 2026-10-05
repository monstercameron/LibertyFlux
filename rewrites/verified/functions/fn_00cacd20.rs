// original: 0x00cacd20 CTaskComplexGetOutOfWater::~CTaskComplexGetOutOfWater
/// Destructor step: install the vtable, release the owned field, hand off.
///
/// Takes the object pointer in ECX. Writes the vtable pointer(s) at the
/// object's head, then, when the owned field is non-null, calls the
/// release helper with the field value and clears the field. Transfers
/// control to the chained destructor with the same object pointer,
/// returning whatever that call answers.
export!(thiscall, rw_00cacd20(this: u32) -> u32 {
    unsafe {
        *(this as *mut u32) = relocated(0x00ED89CC);
        let field = *(this.wrapping_add(0x30) as *const u32);
        if field != 0 {
            callee_cdecl!(2, u32, field);
            *(this.wrapping_add(0x30) as *mut u32) = 0;
        }
        callee_thiscall!(1, u32, this)
    }
});
