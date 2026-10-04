// original: 0x00694f40 filter_init
/// Initialise a filter object: vtable pointer plus zeroed slots.
export!(thiscall, rs80_694f40(this: *mut u8) -> u32 {
    unsafe {
        *((this).add(0) as *mut u32) = relocated(0x00FE3994);
        *((this).add(4) as *mut u32) = 0;
        *((this).add(8) as *mut u32) = 0;
        *((this).add(0x0C) as *mut u32) = 0;
        *((this).add(0x10) as *mut u32) = 0;
        *((this).add(0x14) as *mut u32) = 0;
        this as u32
    }
});
