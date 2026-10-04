// original: 0x0094caf0 reset_slot_array_16
/// Fill a 16-slot table with the invalid marker.
///
/// Writes `0xFFFFFFFF` to all 16 dwords at `slots`. Returns nothing
/// meaningful (the original leaves EAX untouched).
export!(thiscall, rw_0094caf0(slots: *mut u32) -> u32 {
    unsafe {
        for i in 0..16 {
            *slots.add(i) = 0xFFFF_FFFF;
        }
        0
    }
});
