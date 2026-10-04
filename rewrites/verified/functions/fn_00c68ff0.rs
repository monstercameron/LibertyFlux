// original: 0x00c68ff0 slot_array_contains
// Linear search of the 64-slot array for a value. Returns 1 when present,
// else 0.
export!(thiscall, rw_00c68ff0(slots: *const u32, v: u32) -> u32 {
    unsafe {
        for i in 0..64u32 {
            if *slots.add(i as usize) == v {
                return 1;
            }
        }
        0
    }
});
