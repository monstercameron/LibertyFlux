// original: 0x00ad0120 zero_wheel_field_block
/// Zero two triples of fields on the object.
///
/// Writes zero to the dwords at 0xf0..0xf8 and 0x100..0x108. The original
/// leaves the return register untouched, so there is no meaningful return.
export!(thiscall, rw_00ad0120(this: *mut u8) -> u32 {
    unsafe {
        *(this.add(0xf8) as *mut u32) = 0;
        *(this.add(0xf4) as *mut u32) = 0;
        *(this.add(0xf0) as *mut u32) = 0;
        *(this.add(0x108) as *mut u32) = 0;
        *(this.add(0x104) as *mut u32) = 0;
        *(this.add(0x100) as *mut u32) = 0;
        0
    }
});
