// original: 0x00cca0a0 CTaskComplexMoveAboutInjured::CTaskComplexMoveAboutInjured
/// Move-about-injured-task constructor: base-construct, stamp the vtable,
/// store the parameter at +0x14, clear +0x18, set +0x1C to all-ones, and
/// when the parameter is non-null run the handle helper (thiscall/1, id 2)
/// with (arg0, &field_14). Returns `this`.
export!(thiscall, rw_00cca0a0(this: *mut u8, a0: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xEDA084;
        let _: u32 = callee_thiscall!(1, u32, this as u32);
        *(this as *mut u32) = relocated(VTABLE);
        *((this.add(0x14)) as *mut u32) = a0;
        *((this.add(0x18)) as *mut u32) = 0;
        *((this.add(0x1C)) as *mut u32) = 0xFFFF_FFFF;
        if a0 != 0 {
            let field = (this.add(0x14)) as u32;
            let _: u32 = callee_thiscall!(2, u32, a0, field);
        }
        this as u32
    }
});
