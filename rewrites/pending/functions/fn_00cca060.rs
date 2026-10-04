// original: 0x00cca060 CTaskComplexInjuredOnGround::CTaskComplexInjuredOnGround
/// Injured-on-ground-task constructor: base-construct, clear flag bit 0 at
/// +0x24, store the two parameters at +0x1C/+0x20, stamp the vtable, clear
/// +0x14/+0x18/+0x28. Returns `this`.
export!(thiscall, rw_00cca060(this: *mut u8, a0: u32, a1: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xED9E74;
        let _: u32 = callee_thiscall!(1, u32, this as u32);
        *this.add(0x24) &= 0xFE;
        *((this.add(0x1C)) as *mut u32) = a0;
        *((this.add(0x20)) as *mut u32) = a1;
        *(this as *mut u32) = relocated(VTABLE);
        *((this.add(0x14)) as *mut u32) = 0;
        *this.add(0x18) = 0;
        *((this.add(0x28)) as *mut u32) = 0;
        this as u32
    }
});
