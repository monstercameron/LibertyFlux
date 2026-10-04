// original: 0x00db1690 clear_three_state_flags
/// Clear three state bytes spread across the frame object.
///
/// Each flag lives in a different region of the object; all three are
/// reset to zero. Takes no arguments beyond the object pointer.
export!(thiscall, rw_00db1690(this_ptr: u32) -> u32 {
    unsafe {
        const FLAG_OFFSETS: [usize; 3] = [0x130, 0x174, 0x1b8];
        let base = this_ptr as *mut u8;
        let mut i = 0;
        while i < FLAG_OFFSETS.len() {
            *base.add(FLAG_OFFSETS[i]) = 0;
            i += 1;
        }
        0
    }
});
