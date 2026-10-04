// original: 0x0089c300 rage::audSpeechSound::audSpeechSound
/// `rage::audSpeechSound` constructor: base-construct, install vtable, init fields.
/// thiscall/0, returns `this`.
export!(thiscall, rw_0089c300(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xE79E9C;
        const FLAG: u32 = 0xB0;
        const MODE: u32 = 0xB4;
        
        callee_thiscall!(1, u32, this);
        *(this as *mut u32) = relocated(VTABLE);
        *((this + FLAG) as *mut u32) = 0xFFFF_FFFF;
        *((this + MODE) as *mut u8) = 0xFF;
        this
    }
});
