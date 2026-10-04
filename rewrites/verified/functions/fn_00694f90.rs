// original: 0x00694f90 filter_init_attach
/// Initialise a filter object, then attach it to the given owner record.
export!(thiscall, rs80_694f90(this: *mut u8, owner: u32) -> u32 {
    unsafe {
        *((this).add(0) as *mut u32) = relocated(0x00FE3994);
        *((this).add(4) as *mut u32) = 0;
        *((this).add(8) as *mut u32) = 0;
        *((this).add(0x0C) as *mut u32) = 0;
        *((this).add(0x10) as *mut u32) = 0;
        *((this).add(0x14) as *mut u32) = 0;
        let _: u32 = callee_thiscall!(1, u32, this as u32, owner);
        this as u32
    }
});
