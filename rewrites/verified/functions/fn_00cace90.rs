// original: 0x00cace90 CTaskComplexMoveFollowPointRoute::~CTaskComplexMoveFollowPointRoute
/// Destructor step: install the vtable, release through shared context, hand off.
///
/// Takes the object pointer in ECX. Writes the vtable pointer(s) at the
/// object's head, then, when the owned field is non-null, calls the
/// shared release entry with the context object and the field value.
/// Transfers control to the chained destructor with the same object
/// pointer, returning whatever that call answers.
export!(thiscall, rw_00cace90(this: u32) -> u32 {
    unsafe {
        *(this as *mut u32) = relocated(0x00ED85BC);
        *(this.wrapping_add(0x14) as *mut u32) = relocated(0x00ED8618);
        let field = *(this.wrapping_add(0x34) as *const u32);
        if field != 0 {
            let ctx = *(global::<u32>(0x0179D114) as *const u32);
            callee_thiscall!(2, u32, ctx, field);
        }
        callee_thiscall!(1, u32, this)
    }
});
