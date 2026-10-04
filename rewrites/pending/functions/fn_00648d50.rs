// original: 0x00648d50 rage__rmPtfxShaderVar_Float3__ctor (proposed)
/// Construct a three-float particle shader variable: zero the links, set
/// default flags, install the class vtable, clear the three value words and
/// tag the kind.
export!(thiscall, rw_00648d50(this: *mut u8) -> u32 {
    unsafe {
        *(this.add(0x08) as *mut u32) = 0;
        *(this.add(0x04) as *mut u32) = 0;
        *(this.add(0x10) as *mut u16) = 0x0100;
        *(this as *mut u32) = relocated(0x00FE2BFC);
        *(this.add(0x20) as *mut u32) = 0;
        *(this.add(0x24) as *mut u32) = 0;
        *(this.add(0x28) as *mut u32) = 0;
        *this.add(0x12) = 4;
        this as u32
    }
});
