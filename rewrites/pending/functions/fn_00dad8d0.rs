// original: 0x00dad8d0 CTaskComplexAvoidPlayerTargetting::CTaskComplexAvoidPlayerTargetting_3
/// Constructor: run the base constructor, install the vtable, store the two
/// reference arguments at +0x14/+0x18 (the third stack argument is
/// ignored), set the flag bytes, stamp the clock global at +0x34, and
/// register each non-null reference. Returns the object.
export!(thiscall, rw_00dad8d0(this: u32, a: u32, b: u32, _ignored: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00EF07E4;
        let _: u32 = callee_thiscall!(1, u32, this);
        *(this as *mut u32) = relocated(VTABLE);
        *((this as *mut u32).byte_add(0x14)) = a;
        *((this as *mut u32).byte_add(0x18)) = b;
        *((this as *mut u8).byte_add(0x1C)) = 0;
        *((this as *mut u8).byte_add(0x30)) = 1;
        *((this as *mut u32).byte_add(0x34)) = *global::<u32>(0x011735B4);
        if a != 0 {
            let _: u32 = callee_stdcall!(2, u32, this.wrapping_add(0x14));
        }
        if b != 0 {
            let _: u32 = callee_stdcall!(2, u32, this.wrapping_add(0x18));
        }
        let _ = _ignored;
        this
    }
});
