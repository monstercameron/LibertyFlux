// original: 0x00c67820 slot_array_find_or_insert
/// Find `value` in a 64-slot id array, inserting it into the first free slot.
///
/// A slot holding the value ends the search; the first slot holding a
/// negative value counts as free and takes the value. Returns the slot index,
/// or 64 when every slot is taken by a different non-negative value.
export!(thiscall, rw_00c67820(slots: *mut u32, value: u32) -> u32 {
    unsafe {
        for i in 0..64u32 {
            let cur = *slots.add(i as usize);
            if cur == value {
                return i;
            }
            if (cur as i32) < 0 {
                *slots.add(i as usize) = value;
                return i;
            }
        }
        64
    }
});
