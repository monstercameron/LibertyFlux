// original: 0x00648a80 rage__rmPtfxShaderVar_Bool__ctor (proposed)
/// Construct a boolean particle shader variable: zero the links, set default
/// flags, install the class vtable, clear the value byte and tag the kind.
export!(thiscall, rw_00648a80(this: *mut u8) -> u32 {
    unsafe {
        *(this.add(0x08) as *mut u32) = 0;
        *(this.add(0x04) as *mut u32) = 0;
        *(this.add(0x10) as *mut u16) = 0x0100;
        *(this as *mut u32) = relocated(0x00FE2B24);
        *this.add(0x20) = 0;
        *this.add(0x12) = 0;
        this as u32
    }
});
