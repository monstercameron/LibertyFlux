// original: 0x00dad940 CTaskComplexInvestigateDeadPed::CTaskComplexInvestigateDeadPed
/// Constructor: run the base constructor, install the vtable, store the
/// reference argument at +0x14 and the mode word at +0x1C, clear the
/// remaining header fields, and register a non-null reference.
/// Returns the object pointer.
export!(thiscall, rw_00dad940(this: u32, a: u32, b: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00EF06DC;
        let _: u32 = callee_thiscall!(1, u32, this);
        *((this as *mut u32).byte_add(0x14)) = a;
        *(this as *mut u32) = relocated(VTABLE);
        *((this as *mut u8).byte_add(0x18)) = 0;
        *((this as *mut u32).byte_add(0x1C)) = b;
        *((this as *mut u32).byte_add(0x20)) = 0;
        *((this as *mut u32).byte_add(0x24)) = 0;
        *((this as *mut u16).byte_add(0x28)) = 0;
        if a != 0 {
            let _: u32 = callee_stdcall!(2, u32, this.wrapping_add(0x14));
        }
        this
    }
});
