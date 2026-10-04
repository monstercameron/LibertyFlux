// original: 0x00dad860 CTaskComplexAvoidPlayerTargetting::CTaskComplexAvoidPlayerTargetting_2
/// Constructor: run the base constructor, install the vtable, store the two
/// reference arguments at +0x14/+0x18, set the mode byte, copy twelve
/// bytes from the third argument to +0x20..+0x2B, stamp the clock global
/// at +0x34, and register each non-null reference. Returns the object.
export!(thiscall, rw_00dad860(this: u32, a: u32, b: u32, c: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00EF07E4;
        let _: u32 = callee_thiscall!(1, u32, this);
        *((this as *mut u32).byte_add(0x14)) = a;
        *(this as *mut u32) = relocated(VTABLE);
        *((this as *mut u32).byte_add(0x18)) = b;
        *((this as *mut u8).byte_add(0x1C)) = 1;
        let src = c as *const u32;
        let dst = (this as *mut u32).byte_add(0x20);
        *dst.add(0) = *src.add(0);
        *dst.add(1) = *src.add(1);
        *dst.add(2) = *src.add(2);
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
