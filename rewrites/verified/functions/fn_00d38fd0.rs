// original: 0x00d38fd0 flag_latch_store
/// Latch a flag byte and its shadow: always store the incoming value's low
/// byte at +0x33, then set +0x30 unless the object is idle and unflagged.
/// The original defines no return value (checked with ret:none).
export!(thiscall, rw_00d38fd0(this: u32, v: u32) -> u32 {
    unsafe { *((this + 0x33) as *mut u8) = v as u8 };
    if unsafe { *((this + 0x30) as *const u8) } != 0 {
        unsafe { *((this + 0x30) as *mut u8) = 1 };
    } else {
        let flags = unsafe { *((this + 0x14) as *const u32) };
        if flags & 0x10 != 0 {
            unsafe { *((this + 0x30) as *mut u8) = 1 };
        } else if v as u8 == 0 {
            unsafe { *((this + 0x30) as *mut u8) = 0 };
        } else if flags & 0x20 != 0 {
            unsafe { *((this + 0x30) as *mut u8) = 1 };
        } else {
            unsafe { *((this + 0x30) as *mut u8) = 0 };
        }
    }
    0
});
