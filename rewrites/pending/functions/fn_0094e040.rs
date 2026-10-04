// original: 0x0094e040 find_and_invalidate_slot
/// Clear every slot holding a key, reporting whether any matched.
///
/// Scans all 16 slots (no early exit): each slot equal to `key` is set to the
/// invalid marker. Returns 1 when at least one slot matched, else 0.
export!(thiscall, rw_0094e040(slots: *mut u32, key: u32) -> u32 {
    unsafe {
        let mut found = false;
        for i in 0..16 {
            if *slots.add(i) == key {
                *slots.add(i) = 0xFFFF_FFFF;
                found = true;
            }
        }
        found as u32
    }
});
