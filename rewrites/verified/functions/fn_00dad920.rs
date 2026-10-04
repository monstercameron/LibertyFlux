// original: 0x00dad920 CTaskComplexExtinguishFires::CTaskComplexExtinguishFires
/// Constructor: run the base constructor, install the vtable and clear the
/// member at +0x14. Returns the object pointer.
export!(thiscall, rw_00dad920(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00EF078C;
        let _: u32 = callee_thiscall!(1, u32, this);
        *(this as *mut u32) = relocated(VTABLE);
        *((this as *mut u32).byte_add(0x14)) = 0;
        this
    }
});
