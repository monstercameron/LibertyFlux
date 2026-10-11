// original: 0x00B44F40 CEventAcquaintancePed::CEventAcquaintancePed
/// Construct an acquaintance-ped event in caller-provided storage. The base
/// event constructor is called first; the event vtable and referenced-ped
/// field are then installed. A non-null field is forwarded by address to the
/// reference helper. The method returns the original `this` pointer.
lf_checker_rt::export!(thiscall, rw_00b44f40(this: u32, referenced_ped: u32) -> u32 {
    unsafe {
        const EVENT_VTABLE: u32 = 0x00eadf3c;
        const REFERENCED_PED: usize = 0x18;
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
        let object = this as *mut u8;
        let field = object.add(REFERENCED_PED);
        (object as *mut u32).write_unaligned(lf_checker_rt::relocated(EVENT_VTABLE));
        (field as *mut u32).write_unaligned(referenced_ped);
        if referenced_ped != 0 {
            let _ = lf_checker_rt::callee_stdcall!(2, u32, field as u32);
        }
        this
    }
});
