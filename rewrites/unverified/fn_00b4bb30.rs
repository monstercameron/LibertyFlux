// original: 0X00B4BB30 group_init_member_slots

/// Initialize the group's eight member records. Each record stores a null
/// member pointer and a signed negative empty marker; the trailing distance
/// default is the exact 30.0f bit pattern. The object is supplied in ECX and
/// is returned unchanged.
lf_checker_rt::export!(thiscall, rw_00B4BB30(object: u32) -> u32 {
    const FIRST_MEMBER: u32 = 0x04;
    const MEMBER_COUNT: u32 = 8;
    const MEMBER_STRIDE: u32 = 8;
    const EMPTY_MARKER: u32 = u32::MAX;
    const DEFAULT_DISTANCE_BITS: u32 = 0x41F0_0000;

    for index in 0..MEMBER_COUNT {
        let slot = object.wrapping_add(FIRST_MEMBER + index * MEMBER_STRIDE);
        // SAFETY: the supplied group object covers all eight member records.
        unsafe {
            (slot as *mut u32).write_unaligned(0);
            (slot.wrapping_add(4) as *mut u32).write_unaligned(EMPTY_MARKER);
        }
    }
    // SAFETY: the group layout includes the trailing default-distance field.
    unsafe { (object.wrapping_add(0x44) as *mut u32).write_unaligned(DEFAULT_DISTANCE_BITS) };
    object
});
