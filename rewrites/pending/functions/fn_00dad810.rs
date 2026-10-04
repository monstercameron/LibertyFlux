// original: 0x00dad810 CTaskComplexAvoidPlayerTargetting::CTaskComplexAvoidPlayerTargetting
/// Constructor: run the base constructor, install the vtable, store the two
/// reference arguments at +0x14/+0x18, clear the flag bytes, stamp the
/// clock global at +0x34, and register each non-null reference through
/// the reference helper. Returns the object pointer.
export!(thiscall, rw_00dad810(this: u32, a: u32, b: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00EF07E4;
        let _: u32 = callee_thiscall!(1, u32, this);
        *(this as *mut u32) = relocated(VTABLE);
        *((this as *mut u32).byte_add(0x14)) = a;
        *((this as *mut u32).byte_add(0x18)) = b;
        *((this as *mut u8).byte_add(0x1C)) = 0;
        *((this as *mut u8).byte_add(0x30)) = 0;
        *((this as *mut u32).byte_add(0x34)) = *global::<u32>(0x011735B4);
        if a != 0 {
            let _: u32 = callee_stdcall!(2, u32, this.wrapping_add(0x14));
        }
        if b != 0 {
            let _: u32 = callee_stdcall!(2, u32, this.wrapping_add(0x18));
        }
        this
    }
});
