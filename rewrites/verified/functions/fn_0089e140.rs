// original: 0x0089e140 rage::audSequentialSound::audSequentialSound
/// `rage::audSequentialSound` constructor: base-construct, install vtable, init fields.
/// thiscall/0, returns `this`.
export!(thiscall, rw_0089e140(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xE7A2EC;
        const ONE: u32 = 0x3F80_0000; // 1.0f
        
        callee_thiscall!(1, u32, this);
        *(this as *mut u32) = relocated(VTABLE);
        *((this + 0xB0) as *mut u32) = 0xFFFF_FFFF;
        *((this + 0xB4) as *mut u32) = 0xFFFF_FFFF;
        *((this + 0xB8) as *mut u32) = ONE;
        *((this + 0xBC) as *mut u32) = ONE;
        *((this + 0xC0) as *mut u32) = 0;
        *((this + 0xC4) as *mut u16) = 0;
        *((this + 0xCC) as *mut u32) = 0xFFFF_FFFF;
        *((this + 0xD0) as *mut u16) = 0xFFFF;
        this
    }
});
