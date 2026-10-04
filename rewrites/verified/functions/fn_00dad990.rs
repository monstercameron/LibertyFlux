// original: 0x00dad990 CTaskComplexReactToGunAimedAt::CTaskComplexReactToGunAimedAt
/// Constructor: run the base constructor, install the vtable, store the
/// reference argument at +0x14, clear the word at +0x18, and register
/// a non-null reference. Returns the object pointer.
export!(thiscall, rw_00dad990(this: u32, a: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00EF0734;
        let _: u32 = callee_thiscall!(1, u32, this);
        *((this as *mut u32).byte_add(0x14)) = a;
        *(this as *mut u32) = relocated(VTABLE);
        *((this as *mut u32).byte_add(0x18)) = 0;
        if a != 0 {
            let _: u32 = callee_stdcall!(2, u32, this.wrapping_add(0x14));
        }
        this
    }
});
