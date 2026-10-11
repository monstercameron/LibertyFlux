// original: 0x005DCCB0 CTaskSimpleStartCar::vf14

/// Return the task's stored start-car value when its ready flag is set.
///
/// The method tests bit 3 of the byte at offset 0x18. If that bit is clear,
/// it returns zero; otherwise it returns the 32-bit value at offset 0x14.
/// The object is read-only and uses the game's 32-bit task layout.
lf_checker_rt::export!(thiscall, rw_005dccb0(this: u32) -> u32 {
    const VALUE_OFFSET: u32 = 0x14;
    const FLAGS_OFFSET: u32 = 0x18;
    const READY_FLAG: u8 = 0x08;

    let flags = unsafe {
        (this.wrapping_add(FLAGS_OFFSET) as *const u8).read()
    };
    if flags & READY_FLAG == 0 {
        0
    } else {
        unsafe {
            (this.wrapping_add(VALUE_OFFSET) as *const u32).read_unaligned()
        }
    }
});
