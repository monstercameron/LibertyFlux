// original: 0x0089d5e0 rage::audOnStopSound::audOnStopSound
/// `rage::audOnStopSound` constructor: base-construct, install vtable, zero fields.
/// thiscall/0, returns `this`.
export!(thiscall, rw_0089d5e0(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xE7A020;
        let base: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        base(this);
        *(this as *mut u32) = relocated(VTABLE);
        *((this + 0xB0) as *mut u16) = 0;
        *((this + 0xB2) as *mut u8) = 0;
        this
    }
});
