// original: 0x00a64b50 area_record_setup_single
/// Sets up the single area record with a zeroed second record.
///
/// Registers the flag through the area helpers, sets flag bit 0, copies
/// the source record and the trailing float, zeroes the second record and
/// sets flag bit 2. Returns nothing.
export!(thiscall, rw_00a64b50(this: u32, src: u32, farg: u32, flag: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this);
        if flag != 0 {
            *((this + 0x24) as *mut u32) = flag;
            callee_thiscall!(2, u32, flag, this + 0x24);
        }
        *((this + 0x28) as *mut u32) |= 1;
        *((this) as *mut u32) = *(src as *const u32);
        *((this + 4) as *mut u32) = *((src + 4) as *const u32);
        *((this + 8) as *mut u32) = *((src + 8) as *const u32);
        *((this + 0xC) as *mut u32) = *((src + 0xC) as *const u32);
        *((this + 0x20) as *mut u32) = farg;
        *((this + 0x10) as *mut u32) = 0;
        *((this + 0x14) as *mut u32) = 0;
        *((this + 0x18) as *mut u32) = 0;
        *((this + 0x28) as *mut u32) |= 4;
    }
    0
});
