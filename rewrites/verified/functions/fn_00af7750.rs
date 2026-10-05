// original: 0x00AF7750 veh_handles_reset_9 (proposed)

/// Reset nine consecutive handle slots to the empty marker.
///
/// Writes `0xFFFFFFFF` to the nine dwords at `this + 0x14` through
/// `this + 0x34` stepping by 4. Nothing is read and no value is returned.
///
/// Original: 0x00AF7750 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00AF7750(this: u32) -> u32 {
    unsafe {
        const FIRST_SLOT: u32 = 0x14;
        const SLOT_COUNT: u32 = 9;
        const EMPTY: u32 = 0xFFFF_FFFF;
        for i in 0..SLOT_COUNT {
            ((this + FIRST_SLOT + i * 4) as *mut u32).write_unaligned(EMPTY);
        }
        0
    }
});
