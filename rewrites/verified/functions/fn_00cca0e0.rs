// original: 0x00cca0e0 CTaskComplexOnFire::CTaskComplexOnFire
/// On-fire-task constructor: base-construct, store the parameter at +0x18,
/// stamp the vtable, clear +0x14. Returns `this`.
export!(thiscall, rw_00cca0e0(this: *mut u8, a0: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xEDA02C;
        let _: u32 = callee_thiscall!(1, u32, this as u32);
        *((this.add(0x18)) as *mut u32) = a0;
        *(this as *mut u32) = relocated(VTABLE);
        *((this.add(0x14)) as *mut u32) = 0;
        this as u32
    }
});
