// original: 0x00AF7910 veh_last_active_slot (proposed)

/// Find the highest slot whose entry word is not the empty marker.
///
/// Scans the words at `this + 4 * i` for `i` from 13 down to 1 and returns
/// the first index whose word differs from `0xFFFF`. Slot 0 is never read:
/// when every scanned word holds the marker the result is 0.
///
/// Original: 0x00AF7910 (thiscall, no stack arguments, returns index in EAX).
lf_checker_rt::export!(thiscall, rw_00AF7910(this: u32) -> u32 {
    unsafe {
        const TOP_SLOT: u32 = 13;
        const EMPTY: u16 = 0xFFFF;
        let mut i = TOP_SLOT;
        while i > 0 {
            if ((this + i * 4) as *const u16).read_unaligned() != EMPTY {
                return i;
            }
            i -= 1;
        }
        0
    }
});
