// original: 0x00648ee0 rage__rmPtfxShaderVar_tag6__ctor (proposed)
/// Construct a kind-tag-6 shader variable (two-word value): zero the links,
/// set default flags, install the class vtable, clear both value words and
/// tag the kind.
export!(thiscall, rw_00648ee0(this: *mut u8) -> u32 {
    unsafe {
        *(this.add(0x08) as *mut u32) = 0;
        *(this.add(0x04) as *mut u32) = 0;
        *(this.add(0x10) as *mut u16) = 0x0100;
        *(this as *mut u32) = relocated(0x00FE2C44);
        *(this.add(0x20) as *mut u32) = 0;
        *(this.add(0x24) as *mut u32) = 0;
        *this.add(0x12) = 6;
        this as u32
    }
});
