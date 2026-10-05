// original: 0x00AF7810 veh_handles_reset_14 (proposed)

/// Reset fourteen consecutive handle slots to the empty marker.
///
/// Writes `0xFFFFFFFF` to the fourteen dwords at `this + 0x00` through
/// `this + 0x34` stepping by 4. Nothing is read and no value is returned.
///
/// Original: 0x00AF7810 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00AF7810(this: u32) -> u32 {
    unsafe {
        const SLOT_COUNT: u32 = 14;
        const EMPTY: u32 = 0xFFFF_FFFF;
        for i in 0..SLOT_COUNT {
            ((this + i * 4) as *mut u32).write_unaligned(EMPTY);
        }
        0
    }
});
