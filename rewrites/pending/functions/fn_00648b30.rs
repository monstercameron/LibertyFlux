// original: 0x00648b30 rage__rmPtfxShaderVar_Float__ctor (proposed)
/// Construct a float particle shader variable: zero the links, set default
/// flags, install the class vtable, clear the value word and tag the kind.
export!(thiscall, rw_00648b30(this: *mut u8) -> u32 {
    unsafe {
        *(this.add(0x08) as *mut u32) = 0;
        *(this.add(0x04) as *mut u32) = 0;
        *(this.add(0x10) as *mut u16) = 0x0100;
        *(this as *mut u32) = relocated(0x00FE2BB4);
        *(this.add(0x20) as *mut u32) = 0;
        *this.add(0x12) = 1;
        this as u32
    }
});
