// original: 0x00c67bc0 slot_array_leading_count
/// Count leading taken slots: the number of entries from slot 0 onward that
/// are non-negative, stopping at the first negative entry or after 64 slots.
export!(thiscall, rw_00c67bc0(slots: *const u32) -> u32 {
    unsafe {
        let mut n = 0u32;
        for i in 0..64usize {
            if (*slots.add(i) as i32) < 0 {
                break;
            }
            n += 1;
        }
        n
    }
});
