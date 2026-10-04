// original: 0x00c67b00 slot_array_reset
/// Reset a 64-slot id array: every slot becomes empty (-1).
///
/// Returns -1 as u32 (the fill value the original leaves in EAX).
export!(thiscall, rw_00c67b00(slots: *mut u32) -> u32 {
    unsafe {
        for i in 0..64usize {
            *slots.add(i) = 0xffff_ffff;
        }
        0xffff_ffff
    }
});
